#####
date = "2025-11-20"
author = "Nzuzo Magagula"
summary = "Implementing database backends with trait-based abstraction - designing portable APIs that work seamlessly across Sled, Redb, and IndexedDB"
thumbnail = "https://i.postimg.cc/pdKhS5Rk/netabase-store-architecture.png"
category = "Technical"
show_references = true

[[article_series]]
name = "Building netabase_store"
previous = "netabase_store/02-procedural-macros-and-code-generation"
next = "netabase_store/04-configuration-api-and-transaction-system"
#####
# Building netabase_store: Backend Implementation and Trait Design - Part 3

## Introduction

In the [previous article](./02-procedural-macros-and-code-generation.md), we explored how procedural macros generate type-safe code. Now we'll see how trait-based design enables true backend portability - allowing the same application code to work seamlessly with Sled, Redb, or IndexedDB.

This article covers:
- Designing abstractions that hide backend differences
- Implementing the `NetabaseTreeSync` trait for Sled
- Managing lifetimes for safe resource access
- Handling secondary key indexing
- Cross-platform considerations (native vs WASM)

## The Power of Trait-Based Abstraction

The key insight is that despite their differences, all key-value databases offer similar operations. By defining a trait that captures these operations, we can write code that works with any backend:

```rust
pub trait NetabaseTreeSync<'db, D, M> {
    type PrimaryKey;
    type SecondaryKeys;

    fn put(&self, model: M) -> Result<(), NetabaseError>;
    fn get(&self, key: Self::PrimaryKey) -> Result<Option<M>, NetabaseError>;
    fn remove(&self, key: Self::PrimaryKey) -> Result<Option<M>, NetabaseError>;
    fn get_by_secondary_key(&self, key: Self::SecondaryKeys)
        -> Result<Vec<M>, NetabaseError>;

    fn is_empty(&self) -> Result<bool, NetabaseError>;
    fn len(&self) -> Result<usize, NetabaseError>;
    fn clear(&self) -> Result<(), NetabaseError>;
}
```

### Design Choices

Let's examine the key design decisions:

**1. Associated Types vs Generic Parameters**

```rust
// Why this:
pub trait NetabaseTreeSync<'db, D, M> {
    type PrimaryKey;
    type SecondaryKeys;
}

// Instead of this:
pub trait NetabaseTreeSync<'db, D, M, PK, SK> {
    fn get(&self, key: PK) -> Result<Option<M>, NetabaseError>;
}
```

Associated types make the API cleaner. Each model has exactly one primary key type and one secondary keys enum, so they should be associated with the implementation rather than chosen at call sites.

**2. Borrowing Strategy**

```rust
fn put(&self, model: M) -> Result<(), NetabaseError>;
```

We take `model` by value (consuming it) because:
- Models are typically cloned from user code
- We need to extract keys, which requires owned values
- It makes the ownership semantics clear

**3. Secondary Keys Return Vec**

```rust
fn get_by_secondary_key(&self, key: Self::SecondaryKeys)
    -> Result<Vec<M>, NetabaseError>;
```

Unlike primary keys (which are unique), multiple models can have the same secondary key value. Returning `Vec<M>` makes this explicit.

## Implementing SledStore

Sled is a high-performance embedded database. Let's see how we implement the traits for it.

### The Store Structure

```rust
pub struct SledStore<D>
where
    D: NetabaseDefinitionTrait,
{
    pub(crate) db: sled::Db,
    pub trees: Vec<D::Discriminant>,
}
```

The store holds:
- A sled database instance
- A list of all known tree discriminants (for iteration)

### The Tree Structure

Each model gets its own "tree" (sled's term for a namespace within the database):

```rust
pub struct SledStoreTree<'db, D, M>
where
    D: NetabaseDefinitionTrait,
    M: NetabaseModelTrait<D>,
{
    pub(crate) tree: sled::Tree,           // Primary key → model
    pub(crate) secondary_tree: sled::Tree,  // Secondary key → primary key
    pub db: sled::Db,                       // Reference to parent DB
    pub(crate) _phantom_d: PhantomData<D>,
    pub(crate) _phantom_m: PhantomData<M>,
    pub(crate) _phantom_db: PhantomData<&'db ()>,
}
```

**Key observations:**
- We maintain two sled trees: one for primary storage, one for secondary indexes
- Phantom types ensure type safety without runtime cost
- The `'db` lifetime ties the tree to its parent store

### Creating Trees

```rust
impl<D> SledStore<D>
where
    D: NetabaseDefinitionTrait,
{
    pub fn open_tree<M>(&self) -> SledStoreTree<'_, D, M>
    where
        M: NetabaseModelTrait<D>,
    {
        let tree_name = M::discriminant_name();
        SledStoreTree::new(&self.db, tree_name)
    }
}

impl<'db, D, M> SledStoreTree<'db, D, M> {
    pub(crate) fn new(db: &sled::Db, tree_name: &str) -> Self {
        let tree = db.open_tree(tree_name)
            .expect("Failed to open tree");

        let sec_tree_name = format!("{}_secondary", tree_name);
        let secondary_tree = db.open_tree(sec_tree_name)
            .expect("Failed to open secondary tree");

        Self {
            tree,
            secondary_tree,
            db: db.clone(),
            _phantom_d: PhantomData,
            _phantom_m: PhantomData,
            _phantom_db: PhantomData,
        }
    }
}
```

The `open_tree` method uses the model's discriminant (e.g., "User", "Post") as the tree name. This ensures each model type gets its own isolated storage.

## Implementing Put: The Write Path

The `put` operation is the most complex because it must maintain consistency between primary and secondary indexes:

```rust
impl<'db, D, M> NetabaseTreeSync<'db, D, M> for SledStoreTree<'db, D, M>
where
    D: NetabaseDefinitionTrait + From<M>,
    M: NetabaseModelTrait<D> + Clone,
{
    type PrimaryKey = M::PrimaryKey;
    type SecondaryKeys = M::SecondaryKeys;

    fn put(&self, model: M) -> Result<(), NetabaseError> {
        // Step 1: Extract keys from model
        let primary_key = model.primary_key();
        let secondary_keys = model.secondary_keys();

        // Step 2: Serialize keys and model
        let pk_bytes = bincode::encode_to_vec(&primary_key, bincode::config::standard())?;
        let model_bytes = bincode::encode_to_vec(&model, bincode::config::standard())?;

        // Step 3: Check if model already exists (for secondary key cleanup)
        let old_model: Option<M> = self.tree.get(&pk_bytes)?
            .map(|bytes| {
                let (m, _) = bincode::decode_from_slice(&bytes, bincode::config::standard())?;
                Ok::<M, NetabaseError>(m)
            })
            .transpose()?;

        // Step 4: Use a batch to ensure atomicity
        let mut batch = sled::Batch::default();

        // Insert primary record
        batch.insert(pk_bytes.clone(), model_bytes);

        // Step 5: Remove old secondary indexes
        if let Some(old) = old_model {
            for old_sk in old.secondary_keys() {
                let sk_bytes = bincode::encode_to_vec(&old_sk, bincode::config::standard())?;
                batch.remove(sk_bytes);
            }
        }

        // Step 6: Insert new secondary indexes
        for sk in secondary_keys {
            let sk_bytes = bincode::encode_to_vec(&sk, bincode::config::standard())?;
            batch.insert(sk_bytes, pk_bytes.clone());
        }

        // Step 7: Apply batch atomically
        self.tree.apply_batch(batch)?;

        Ok(())
    }
}
```

### Why This Complexity?

1. **Atomicity**: Using a batch ensures all changes (primary + secondary indexes) happen together or not at all
2. **Index Cleanup**: If updating an existing model, we must remove old secondary indexes
3. **Consistency**: Secondary indexes must always point to valid primary keys

### Data Layout

For a `User` model:

```
Primary Tree ("User"):
  [bincode(UserPrimaryKey(1))] → [bincode(User { id: 1, email: "alice@example.com", ... })]

Secondary Tree ("User_secondary"):
  [bincode(UserSecondaryKeys::Email("alice@example.com"))] → [bincode(UserPrimaryKey(1))]
```

## Implementing Get: The Read Path

Reading by primary key is straightforward:

```rust
fn get(&self, key: Self::PrimaryKey) -> Result<Option<M>, NetabaseError> {
    // Serialize the key
    let key_bytes = bincode::encode_to_vec(&key, bincode::config::standard())?;

    // Look up in primary tree
    let value_bytes = self.tree.get(key_bytes)?;

    // Deserialize if found
    match value_bytes {
        Some(bytes) => {
            let (model, _) = bincode::decode_from_slice(
                &bytes,
                bincode::config::standard()
            )?;
            Ok(Some(model))
        }
        None => Ok(None),
    }
}
```

**Performance characteristics:**
- Sled provides O(log n) lookups via B-tree
- Deserialization cost is proportional to model size
- No allocations beyond the model itself

## Implementing Secondary Key Queries

Querying by secondary key requires two lookups:

```rust
fn get_by_secondary_key(&self, key: Self::SecondaryKeys)
    -> Result<Vec<M>, NetabaseError>
{
    let mut results = Vec::new();

    // Step 1: Serialize secondary key
    let sk_bytes = bincode::encode_to_vec(&key, bincode::config::standard())?;

    // Step 2: Scan secondary tree for matches
    for item in self.secondary_tree.scan_prefix(&sk_bytes) {
        let (_, pk_bytes) = item?;

        // Step 3: Look up model by primary key
        if let Some(model_bytes) = self.tree.get(pk_bytes)? {
            let (model, _) = bincode::decode_from_slice(
                &model_bytes,
                bincode::config::standard()
            )?;
            results.push(model);
        }
    }

    Ok(results)
}
```

**Why `scan_prefix`?**

Because we serialize the entire secondary key enum, all records with the same key naturally share a prefix:

```
[bincode(UserSecondaryKeys::Email("alice@example.com"))] = prefix for all alice@ records
[bincode(UserSecondaryKeys::Age(30))] = prefix for all age 30 records
```

## Handling Remove

Removal must also clean up secondary indexes:

```rust
fn remove(&self, key: Self::PrimaryKey) -> Result<Option<M>, NetabaseError> {
    let key_bytes = bincode::encode_to_vec(&key, bincode::config::standard())?;

    // Step 1: Get the model (if it exists)
    let model = match self.tree.get(&key_bytes)? {
        Some(bytes) => {
            let (m, _) = bincode::decode_from_slice(&bytes, bincode::config::standard())?;
            Some(m)
        }
        None => return Ok(None),
    };

    let model = model.unwrap();

    // Step 2: Create batch for atomic deletion
    let mut batch = sled::Batch::default();

    // Remove primary record
    batch.remove(key_bytes);

    // Step 3: Remove all secondary indexes
    for sk in model.secondary_keys() {
        let sk_bytes = bincode::encode_to_vec(&sk, bincode::config::standard())?;
        batch.remove(sk_bytes);
    }

    // Step 4: Apply batch
    self.tree.apply_batch(batch)?;

    Ok(Some(model))
}
```

Returning the deleted model allows users to access its data one last time.

## Cross-Platform: Async Traits for WASM

IndexedDB (browser storage) has an asynchronous API. We define a parallel async trait:

```rust
#[cfg(feature = "wasm")]
#[async_trait(?Send)]
pub trait NetabaseTreeAsync<D, M> {
    type PrimaryKey;
    type SecondaryKeys;

    async fn put(&self, model: M) -> Result<(), NetabaseError>;
    async fn get(&self, key: Self::PrimaryKey) -> Result<Option<M>, NetabaseError>;
    async fn remove(&self, key: Self::PrimaryKey) -> Result<Option<M>, NetabaseError>;
    async fn get_by_secondary_key(&self, key: Self::SecondaryKeys)
        -> Result<Vec<M>, NetabaseError>;

    async fn is_empty(&self) -> Result<bool, NetabaseError>;
    async fn len(&self) -> Result<usize, NetabaseError>;
    async fn clear(&self) -> Result<(), NetabaseError>;
}
```

**Note:** We use `?Send` because JavaScript is single-threaded, so futures don't need to be `Send`.

## Writing Backend-Agnostic Code

With traits in place, we can write code that works with any backend:

```rust
// This function works with SledStore, RedbStore, or any future backend
fn count_users<'a, T>(tree: &T) -> Result<usize, NetabaseError>
where
    T: NetabaseTreeSync<'a, BlogDefinition, User>
{
    tree.len()
}

// Use with Sled
let sled_store = SledStore::<BlogDefinition>::temp()?;
let sled_tree = sled_store.open_tree::<User>();
let sled_count = count_users(&sled_tree)?;

// Use with Redb (same function!)
let redb_store = RedbStore::<BlogDefinition>::temp()?;
let redb_tree = redb_store.open_tree::<User>();
let redb_count = count_users(&redb_tree)?;
```

The function `count_users` is **completely backend-agnostic**. It works with any type implementing `NetabaseTreeSync`.

## Lifetime Management

The `'db` lifetime is crucial for safety:

```rust
pub struct SledStoreTree<'db, D, M> {
    // ...
    pub(crate) _phantom_db: PhantomData<&'db ()>,
}
```

This ties the tree's lifetime to the store's lifetime:

```rust
// ✓ OK: Tree outlived by store
{
    let store = SledStore::<BlogDef>::temp()?;
    let tree = store.open_tree::<User>();
    // Use tree...
}  // Both drop together

// ✗ Compile error: Tree would outlive store
let tree = {
    let store = SledStore::<BlogDef>::temp()?;
    store.open_tree::<User>()
};  // Error: `store` dropped while borrowed
```

The compiler prevents us from using trees after their parent store is dropped!

## Benchmarking Trait Overhead

An important question: does the trait abstraction have runtime cost?

```rust
// Direct sled call
let model_bytes = bincode::encode_to_vec(&model, bincode::config::standard())?;
tree.insert(key, model_bytes)?;

// Through NetabaseTreeSync trait
tree.put(model)?;
```

**Answer: Zero overhead.** The trait methods are monomorphized at compile time, producing identical machine code to hand-written direct calls.

## Redb Implementation Differences

Redb is similar to Sled but with different trade-offs. Key differences in implementation:

```rust
pub struct RedbStoreTree<'db, D, M> {
    db: Arc<redb::Database>,
    table_def: TableDefinition<'static, BincodeWrapper<M::PrimaryKey>, BincodeWrapper<M>>,
    // ...
}
```

1. **Static table definitions**: Redb requires compile-time table definitions
2. **Wrapper types**: We use `BincodeWrapper<T>` to implement Redb's `Value` trait
3. **ACID transactions**: Redb provides stronger consistency guarantees

Despite these differences, the `NetabaseTreeSync` implementation looks nearly identical from the outside.

## Summary

Backend abstraction through traits provides:

1. **Portability**: Write once, run on any backend
2. **Type Safety**: Compiler catches mismatched types
3. **Zero Cost**: No runtime overhead from abstraction
4. **Testability**: Easy to test with different backends
5. **Future-Proof**: New backends integrate seamlessly

The key techniques are:

- Associated types for cleaner APIs
- Lifetime parameters for resource safety
- Phantom types for zero-cost type tracking
- Batch operations for atomic consistency
- Careful serialization for cross-backend compatibility

## What's Next?

In the next article, we'll explore the configuration API and transaction system - how we provide a unified, type-safe way to configure different backends and manage multi-operation transactions efficiently.

---

**Further Reading:**
- [Rust API Guidelines: Traits](https://rust-lang.github.io/api-guidelines/future-proofing.html#traits)
- [Associated Types vs Generic Parameters](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types)
- [Phantom Data Explained](https://doc.rust-lang.org/std/marker/struct.PhantomData.html)
- [Sled Documentation](https://docs.rs/sled)
- [Redb Documentation](https://docs.rs/redb)
