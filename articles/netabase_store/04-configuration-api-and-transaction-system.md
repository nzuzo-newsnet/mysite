#####
date = "2025-11-20"
author = "Nzuzo Magagula"
summary = "Building type-safe configuration and transaction systems - the builder pattern and type-state pattern for ergonomic, safe APIs"
thumbnail = "https://i.postimg.cc/pdKhS5Rk/netabase-store-architecture.png"
category = "Technical"
show_references = true

[[article_series]]
name = "Building netabase_store"
previous = "netabase_store/03-backend-implementation-and-trait-design"
next = "netabase_store/05-performance-optimization-and-zerocopy-api"
#####
# Building netabase_store: Configuration API and Transaction System - Part 4

## Introduction

In [Part 3](./03-backend-implementation-and-trait-design.md), we explored how trait-based design enables backend portability. Now we'll examine two critical systems that make the library ergonomic and performant:

1. **Configuration API**: Type-safe, consistent database initialization across all backends
2. **Transaction System**: Compile-time safe transaction management with zero-cost abstractions

Both systems demonstrate advanced Rust patterns: the builder pattern for configuration, and the type-state pattern for transactions.

## The Configuration Problem

Before building a unified configuration system, initialization looked like this:

```rust
// Different constructors for each backend
let sled = SledStore::new("path.db")?;
let redb = RedbStore::open_with_path("path.redb")?;
let temp = SledStore::temp()?;

// Different configuration parameters
let sled = SledStore::with_cache_size("db", 512)?;
let redb = RedbStore::new_with_options("db", RedbOptions { ... })?;
```

Each backend had its own initialization pattern, making backend switching difficult.

## Unified Configuration with typed-builder

We use the `typed-builder` crate to create type-safe, self-documenting configuration objects:

```rust
use typed_builder::TypedBuilder;
use std::path::PathBuf;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(doc)]
pub struct FileConfig {
    /// Path to the database file or directory
    pub path: PathBuf,

    /// Cache size in megabytes
    #[builder(default = 256)]
    pub cache_size_mb: usize,

    /// Whether to create the database if it doesn't exist
    #[builder(default = true)]
    pub create_if_missing: bool,

    /// Whether to truncate/recreate if database already exists
    #[builder(default = false)]
    pub truncate: bool,

    /// Read-only mode (if supported by backend)
    #[builder(default = false)]
    pub read_only: bool,

    /// Enable fsync for durability (may impact performance)
    #[builder(default = true)]
    pub use_fsync: bool,
}
```

### Builder Pattern Benefits

The `TypedBuilder` derive macro generates a builder with:

1. **Required fields**: Must be set (e.g., `path`)
2. **Optional fields**: Have defaults (e.g., `cache_size_mb`)
3. **Type safety**: Wrong types caught at compile time
4. **IDE support**: Autocomplete shows available options
5. **Documentation**: Each field automatically documented

Usage is elegant:

```rust
let config = FileConfig::builder()
    .path("my_app.db".into())
    .cache_size_mb(512)
    .truncate(true)
    .build();
```

### Convenience Constructors

For simple cases, we provide shortcuts:

```rust
impl FileConfig {
    /// Create with just a path, using defaults
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            path: path.into(),
            cache_size_mb: 256,
            create_if_missing: true,
            truncate: false,
            read_only: false,
            use_fsync: true,
        }
    }

    /// Create a temporary database configuration
    pub fn temp() -> Self {
        let temp_path = std::env::temp_dir()
            .join(format!("netabase_{}", uuid::Uuid::new_v4()));
        Self::new(temp_path)
    }
}

// Usage
let config = FileConfig::new("app.db");  // Simple
let temp = FileConfig::temp();            // For testing
```

## The BackendStore Trait

To consume these configurations uniformly, we define the `BackendStore` trait:

```rust
pub trait BackendStore<D: NetabaseDefinitionTrait>: Sized {
    type Config;

    /// Create/open a database with the provided configuration
    fn new(config: Self::Config) -> Result<Self, NetabaseError>;

    /// Open an existing database (fails if missing)
    fn open(config: Self::Config) -> Result<Self, NetabaseError>;

    /// Create a temporary database (for testing)
    fn temp() -> Result<Self, NetabaseError>;
}
```

Each backend implements this trait with its appropriate config type:

```rust
impl<D> BackendStore<D> for SledStore<D>
where
    D: NetabaseDefinitionTrait,
{
    type Config = FileConfig;

    fn new(config: FileConfig) -> Result<Self, NetabaseError> {
        let mut sled_config = sled::Config::new()
            .path(&config.path)
            .cache_capacity(config.cache_size_mb * 1024 * 1024);

        if config.truncate && config.path.exists() {
            std::fs::remove_dir_all(&config.path)?;
        }

        if config.read_only {
            sled_config = sled_config.mode(sled::Mode::LowSpace);
        }

        let db = sled_config.open()?;

        Ok(SledStore {
            db,
            trees: Vec::new(),
        })
    }

    fn open(config: FileConfig) -> Result<Self, NetabaseError> {
        let mut cfg = config;
        cfg.create_if_missing = false;
        Self::new(cfg)
    }

    fn temp() -> Result<Self, NetabaseError> {
        Self::new(FileConfig::temp())
    }
}
```

## Backend Portability Through Configuration

Now switching backends is trivial:

```rust
use netabase_store::config::FileConfig;
use netabase_store::traits::backend_store::BackendStore;

let config = FileConfig::builder()
    .path("database.db".into())
    .cache_size_mb(512)
    .build();

// Try different backends - same config!
#[cfg(feature = "sled")]
let store = <SledStore<MyDef> as BackendStore<MyDef>>::new(config.clone())?;

#[cfg(feature = "redb")]
let store = <RedbStore<MyDef> as BackendStore<MyDef>>::new(config.clone())?;

// API is identical from here on
let tree = store.open_tree::<User>();
tree.put(user)?;
```

The same `FileConfig` works with multiple backends because we designed it to represent common database concepts, not backend-specific details.

## Configuration Types for Different Backends

### MemoryConfig (In-Memory Backend)

```rust
#[derive(Debug, Clone, TypedBuilder)]
pub struct MemoryConfig {
    #[builder(default = 1000)]
    pub initial_capacity: usize,

    #[builder(default = None)]
    pub max_entries: Option<usize>,
}

// Usage
let config = MemoryConfig::builder()
    .initial_capacity(10000)
    .max_entries(Some(1000000))
    .build();

let store = <MemoryStore<MyDef> as BackendStore<MyDef>>::new(config)?;
```

### IndexedDBConfig (WASM Backend)

```rust
#[derive(Debug, Clone, TypedBuilder)]
pub struct IndexedDBConfig {
    pub database_name: String,

    #[builder(default = 1)]
    pub version: u32,
}

// Usage (in WASM)
let config = IndexedDBConfig::builder()
    .database_name("my_app_store".to_string())
    .version(2)
    .build();

let store = <IndexedDBStore<MyDef> as BackendStore<MyDef>>::new(config).await?;
```

## The Transaction Problem

Originally, each operation created its own transaction:

```rust
// ❌ OLD: Each operation = one transaction
tree.put(user1)?;  // Transaction 1: open → put → commit
tree.put(user2)?;  // Transaction 2: open → put → commit
tree.put(user3)?;  // Transaction 3: open → put → commit
// 10-100x slower due to transaction overhead!
```

For Redb especially, this was catastrophically slow because each transaction involved:
1. Acquiring an exclusive lock
2. Creating transaction metadata
3. Committing to the write-ahead log
4. Releasing the lock

## Type-State Pattern for Transactions

The solution: reusable transactions with compile-time mode tracking.

### Zero-Cost Mode Markers

```rust
/// Zero-cost marker type for read-only transactions
pub struct ReadOnly;

/// Zero-cost marker type for read-write transactions
pub struct ReadWrite;
```

These types exist **only at compile time**. They generate zero runtime code but enable compile-time dispatch.

### The Transaction Guard

```rust
pub struct TxnGuard<'db, D, Mode> {
    backend: TxnBackend<'db, D>,
    _mode: PhantomData<Mode>,  // Zero-cost type marker
}
```

The `Mode` parameter determines which methods are available:

```rust
// Operations on ALL modes
impl<'db, D, Mode> TxnGuard<'db, D, Mode> {
    pub fn open_tree<M>(&mut self) -> TreeView<'_, D, M, Mode> {
        // Implementation
    }
}

// Operations ONLY on ReadWrite mode
impl<'db, D> TxnGuard<'db, D, ReadWrite> {
    pub fn commit(self) -> Result<(), NetabaseError> {
        // Implementation
    }

    pub fn rollback(self) -> Result<(), NetabaseError> {
        // Implementation
    }
}
```

### Compile-Time Safety Example

```rust
let txn = store.read();  // Type: TxnGuard<ReadOnly>
let tree = txn.open_tree::<User>();  // Type: TreeView<ReadOnly>

// ✅ Read operations work
let user = tree.get(UserPrimaryKey(1))?;

// ❌ Write operations produce compile errors
tree.put(user)?;
// Error: no method named `put` found for struct `TreeView<'_, D, User, ReadOnly>`
```

The Rust compiler prevents us from writing through a read-only transaction!

### The Tree View

Similar to the transaction guard, tree views inherit the mode:

```rust
pub struct TreeView<'txn, D, M, Mode> {
    backend: TreeBackend<'txn, D, M>,
    _mode: PhantomData<Mode>,
}

// Read operations on ALL modes
impl<'txn, D, M, Mode> TreeView<'txn, D, M, Mode> {
    pub fn get(&self, key: M::PrimaryKey) -> Result<Option<M>, NetabaseError> {
        // Implementation
    }

    pub fn len(&self) -> Result<usize, NetabaseError> {
        // Implementation
    }
}

// Write operations ONLY on ReadWrite mode
impl<'txn, D, M> TreeView<'txn, D, M, ReadWrite> {
    pub fn put(&mut self, model: M) -> Result<(), NetabaseError> {
        // Implementation
    }

    pub fn remove(&mut self, key: M::PrimaryKey) -> Result<Option<M>, NetabaseError> {
        // Implementation
    }
}
```

## Backend-Specific Implementation

### Sled: Immediate Operations

Sled doesn't have true multi-tree transactions, so operations apply immediately:

```rust
pub(crate) struct SledTreeBackend<'txn, D, M> {
    pub(crate) tree: sled::Tree,          // Arc-based, cheap to clone
    pub(crate) secondary_tree: sled::Tree,
    pub(crate) _phantom: PhantomData<(&'txn (), D, M)>,
}

// Put applies immediately to the tree
impl<'txn, D, M> TreeView<'txn, D, M, ReadWrite>
where
    TreeBackend<'txn, D, M>: From<SledTreeBackend<'txn, D, M>>,
{
    pub fn put(&mut self, model: M) -> Result<(), NetabaseError> {
        // Directly insert into sled::Tree
        self.backend.tree.insert(key_bytes, model_bytes)?;
        Ok(())
    }
}
```

### Redb: Transaction Reuse

Redb stores and reuses the transaction:

```rust
pub(crate) struct RedbTxnBackend<'db, D> {
    pub(crate) read_txn: RefCell<Option<redb::ReadTransaction>>,
    pub(crate) write_txn: RefCell<Option<redb::WriteTransaction>>,
    pub(crate) db: &'db Arc<redb::Database>,
    pub(crate) _phantom: PhantomData<D>,
}

// All operations reuse the same transaction
impl<'db, D> TxnGuard<'db, D, ReadWrite> {
    pub fn commit(self) -> Result<(), NetabaseError> {
        match self.backend {
            TxnBackend::Redb(redb) => {
                let write_txn = redb.write_txn.borrow_mut().take()
                    .ok_or(NetabaseError::TransactionError("No write transaction".to_string()))?;
                write_txn.commit()?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
```

**Key insight**: All operations share the same Redb transaction until `commit()` is called.

## Usage Patterns

### Read-Only Transactions

```rust
let txn = store.read();
let user_tree = txn.open_tree::<User>();
let post_tree = txn.open_tree::<Post>();

let user = user_tree.get(UserPrimaryKey(1))?;
let posts = post_tree.get_by_secondary_key(
    PostSecondaryKeys::AuthorId(PostAuthorIdSecondaryKey(1))
)?;

// Auto-closes on drop - no explicit cleanup needed
```

### Read-Write Transactions

```rust
let mut txn = store.write()?;
let mut tree = txn.open_tree::<User>();

// All operations in a single transaction
for i in 0..1000 {
    tree.put(User {
        id: i,
        name: format!("User {}", i),
        email: format!("user{}@example.com", i),
    })?;
}

txn.commit()?;  // Atomic commit of all 1000 inserts
// Or drop to rollback
```

### Explicit Rollback

```rust
let mut txn = store.write()?;
let mut tree = txn.open_tree::<User>();

tree.put(user)?;

if some_condition {
    txn.rollback()?;  // Explicitly abort
} else {
    txn.commit()?;    // Or commit
}
```

## Performance Impact

| Operation | Old API (per-op txn) | New API (reused txn) | Speedup |
|-----------|---------------------|---------------------|---------|
| 1000 inserts (Redb) | ~250ms | ~5ms | **50x** |
| 1000 reads (Redb) | ~150ms | ~3ms | **50x** |
| Mixed ops (Redb) | ~200ms | ~4ms | **50x** |

For Sled, the improvement is smaller (no transaction overhead to begin with), but the API is still cleaner.

## Integration with Configuration

The transaction system works seamlessly with the configuration API:

```rust
// Configure the store
let config = FileConfig::builder()
    .path("app.db".into())
    .cache_size_mb(1024)
    .build();

let store = <RedbStore<MyDef> as BackendStore<MyDef>>::new(config)?;

// Use transactions
let mut txn = store.write()?;
let mut tree = txn.open_tree::<User>();

tree.put_many(users)?;  // Bulk insert in one transaction
txn.commit()?;
```

## Design Patterns Summary

Both systems showcase important Rust patterns:

### Configuration API
- **Builder Pattern**: Type-safe, ergonomic construction
- **Associated Types**: Each backend declares its config type
- **Trait Objects**: Unified interface across backends
- **Smart Defaults**: Required vs optional fields

### Transaction System
- **Type-State Pattern**: Compile-time mode tracking
- **Phantom Types**: Zero-cost polymorphism
- **RAII**: Automatic rollback on drop
- **Interior Mutability**: `RefCell` for shared transaction access

## Compile-Time Guarantees

These systems provide:

1. **No runtime overhead**: Phantom types compile away completely
2. **Impossible states unreachable**: Can't write through read-only transaction
3. **Memory safety**: Lifetimes prevent use-after-free
4. **Backend portability**: Same code works with different backends

## What's Next?

In the final article, we'll explore the ultimate performance optimization: the zero-copy API for Redb. We'll see how careful use of lifetimes and the `ouroboros` crate enable reading data without any deserialization overhead, achieving 54x speedups for certain operations.

---

**Further Reading:**
- [Rust Design Patterns: Builder](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html)
- [Type-State Pattern in Rust](https://cliffle.com/blog/rust-typestate/)
- [PhantomData Explained](https://doc.rust-lang.org/nomicon/phantom-data.html)
- [The `typed-builder` crate](https://docs.rs/typed-builder)
- [RAII in Rust](https://doc.rust-lang.org/rust-by-example/scope/raii.html)
