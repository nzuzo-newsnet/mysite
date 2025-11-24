#####
date = "2025-11-20"
author = "Nzuzo Magagula"
summary = "An introduction to building a type-safe, multi-backend database abstraction library in Rust using procedural macros and trait-based design"
thumbnail = "https://i.postimg.cc/pdKhS5Rk/netabase-store-architecture.png"
category = "Technical"
show_references = true

[[article_series]]
name = "Building netabase_store"
next = "netabase_store/02-procedural-macros-and-code-generation"
#####
# Building netabase_store: A Type-Safe Multi-Backend Database Abstraction — Part 1

## Introduction

This series documents my journey building `netabase_store`, a type-safe, multi-backend key-value storage library in Rust. Instead of trying to write something “authoritative,” I want to share *how I actually stumbled into building this*, what confused me along the way, and why the final design ended up like it did.

If you’ve ever looked at Rust’s procedural macros or wondered how to design a flexible API across different storage backends, maybe my wandering path through this problem will be helpful—or at least entertaining.

## The Problem (A Story of Curiosity and Confusion)

When I started working on `NewsNet`, I came across the [`libp2p`][1] library. If you've never tried it before, it's a great way to dip your toes into [peer-to-peer networking][2]. The thing that stood out to me is how *open-ended* it is—most components give you knobs you can turn in all sorts of ways. That's amazing when you're still learning how everything fits together, but it also meant I spent a *lot* of time exploring and trying different configurations.

My goal was to decentralize as much of `NewsNet` as possible, and I fell down a research rabbit hole that eventually led me to [Kademlia][3] and `libp2p`'s implementation of it. I was especially fascinated by the discovery logic—honestly, that alone made me want to build a prototype.

But once I actually started prototyping, I hit some issues pretty quickly.

### The First Roadblock: Only Bytes Allowed

To plug your own storage backend into `libp2p`'s [DHT][4], you need to implement their [`RecordStore` trait][5]. That part wasn't a problem. What *was* a problem is that `RecordStore` basically only deals with byte arrays. As soon as I needed anything beyond trivial "store and fetch blob" behaviour, things got messy.

I wanted to store richer, typed objects—but trying to manage serialization everywhere by hand kept leading to confusing edge cases. It just didn’t scale cleanly.

### The Second Roadblock: Constant Rituals

`libp2p` gives you an in-memory `RecordStore`, but nothing beyond that. So whenever I wanted to try something slightly more advanced, I had to manually strip out fields like `Instant` that couldn’t be serialized, convert types, redefine structures, and repeat this same ritual over and over while testing different ideas.

Meanwhile, the networking side of things—listening to `Behaviour` events, updating state based on them—was interesting but very repetitive. Any small experiment meant rewriting the same setup code.

### The Two Big Questions

All of this left me staring at two questions:

1. **How do I stop juggling raw bytes everywhere and work with actual, typed data?**
2. **How do I avoid rewriting the same swarm setup code every time I want to try a new experiment?**

These two frustrations planted the seed for what eventually became the `netabase` ecosystem.

## The Spark That Became `netabase`

My idea was simple: I wanted a library that would sit between me and `libp2p`—handling the repetitive parts, flattening out the byte-level details, and letting me focus on actual logic.

`netabase_store` is the first step toward that vision. It answers the first question:
**How can I create type-safe abstractions over key-value stores without writing tons of boilerplate?**

It started with a basic goal—define models once, and let the library generate all the machinery needed to:

* [serialize/deserialize][6] models
* generate typed keys
* create [secondary indexes][7]
* interact with any backend ([sled][8], [redb][9], [IndexedDB][10])
* plug cleanly into the `libp2p` DHT eventually

And, importantly:
**Don’t slow anything down.**
I wanted the abstraction to feel like handwritten Rust, not like something sitting on top adding unnecessary weight.

## Architecture Overview

Here’s how the library is put together, top to bottom, based on what I learned building it.

### 1. Macro Layer

This is where most of the magic happens. I wrote two [procedural macros][11] that generate all the repetitive type definitions and [traits][12] for each model.

```rust
#[netabase_definition_module(BlogDefinition, BlogKeys)]
pub mod blog {
    use netabase_store::{NetabaseModel, netabase};

    #[derive(NetabaseModel, Clone, Debug,
             [bincode][13]::Encode, [bincode][13]::Decode)]
    #[netabase(BlogDefinition)]
    pub struct User {
        #[primary_key]
        pub id: u64,
        pub username: String,
        #[secondary_key]
        pub email: String,
    }
}
```

From this, the macro creates enums, key types, lookup functions—stuff I would *never* want to write by hand.

### 2. Trait Layer

I knew early on that I wanted the same API to work across multiple databases, so the traits ended up forming the backbone of the design.

```rust
pub trait NetabaseTreeSync<D, M> {
    fn put(&self, model: M) -> Result<(), NetabaseError>;
    fn get(&self, key: M::PrimaryKey) -> Result<Option<M>, NetabaseError>;
    fn remove(&self, key: M::PrimaryKey) -> Result<Option<M>, NetabaseError>;
    fn get_by_secondary_key(&self, key: M::SecondaryKeys)
        -> Result<Vec<M>, NetabaseError>;
}
```

A [WASM][14]-compatible async version mirrors it, mainly so IndexedDB can be supported without hacks.

### 3. Backend Layer

Once I had the traits, implementing new backends became straightforward. Each backend just needs to store byte keys and byte values, and the trait layer handles the typed world on top of that.

```rust
pub struct SledStore<D: NetabaseDefinitionTrait> { … }
pub struct RedbStore<D: NetabaseDefinitionTrait> { … }
pub struct IndexedDBStore<D: NetabaseDefinitionTrait> { … }
```

### 4. Unified API Layer

This is the part I wanted from the beginning—a simple, friendly API that hides backend differences:

```rust
let store = NetabaseStore::<BlogDefinition, _>::sled("./data")?;

let user_tree = store.open_tree::<User>();
user_tree.put(user)?;
let retrieved = user_tree.get(UserPrimaryKey(1))?;
```

Same code, three different backends.

## Design Principles I Learned Along the Way

### Zero-Cost Abstractions

Rust's compiler is very kind when you work *with* the type system. All the macro-generated code boils down to efficient plain Rust. The [abstraction stays cheap][15].

### Type-State Pattern

This one surprised me: by encoding "read only" vs "read/write" at the [type level][16], I could prevent accidental writes while a read-only transaction is open.

```rust
let txn = store.read();  
let tree = txn.open_tree::<User>();

tree.get(key)?;   // Works
tree.put(user)?;  // Compile error
```

The compiler becomes a guardrail.

### Automatic Secondary Indexing

This was a big quality-of-life improvement. I didn’t want to manually store extra keys for lookups, so the macro generates everything needed when you annotate fields with `#[secondary_key]`.

## Example Usage

```rust
use netabase_store::traits::model::NetabaseModelTrait;
use netabase_store::traits::store_ops::OpenTree;
use netabase_store::{NetabaseStore, netabase_definition_module};

#[netabase_definition_module(ExampleDefs, ExampleDefKeys)]
pub mod definitions {
    use netabase_store::{NetabaseModel, netabase};

    #[derive(
        NetabaseModel,
        bincode::Encode,
        bincode::Decode,
        Clone,
        Debug,
        serde::Serialize,
        serde::Deserialize,
    )]
    #[netabase(ExampleDefs)]
    pub struct User {
        #[primary_key]
        pub name: String,
        pub age: u8,
        #[secondary_key]
        pub email: String,
    }
}

use definitions::*;

fn main() {
    // Use the unified NetabaseStore API with Sled backend
    let store = NetabaseStore::<ExampleDefs, _>::sled(
        tempfile::tempdir()
            .expect("Failed to create temp dir")
            .path(),
    )
    .expect("The store failed to open");

    let user_tree = store.open_tree::<User>();

    let user = User {
        name: "It's You!".to_string(),
        age: 24,
        email: "some@email.com".to_string(),
    };
    let user2 = User {
        name: "It's Me!".to_string(),
        age: 20,
        email: "some@email.com".to_string(),
    };

    let put_result = user_tree.put(user.clone());

    let get_result = user_tree.get(user.primary_key());

    // Query by secondary key using the model-prefixed type
    let get_secondary_result = user_tree.get_by_secondary_key(UserSecondaryKeys::Email(
        UserEmailSecondaryKey("some@email.com".to_string()),
    ));

    println!("Get Result: {get_result:?}");
    println!("Get Secondary Result: {get_secondary_result:?}");

    assert!(put_result.is_ok());
    assert!(get_result.is_ok());

    let put_result = user_tree.put(user2.clone());

    let get_result = user_tree.get(user2.primary_key());

    // Query by secondary key using the model-prefixed type
    let get_secondary_result = user_tree.get_by_secondary_key(UserSecondaryKeys::Email(
        UserEmailSecondaryKey("some@email.com".to_string()),
    ));

    println!("Get Result: {get_result:?}");
    println!("Get Secondary Result: {get_secondary_result:?}");

    assert!(put_result.is_ok());
    assert!(get_result.is_ok());

    println!("\n✓ Basic store operations completed successfully!");
}
```

## What Makes This Interesting?

The fun part for me was how many different Rust features had to work together:

* procedural macros
* [GATs][17]
* [phantom types][18]
* [conditional compilation][19]
* [zero-copy optimizations][20]
* backend-agnostic traits

I didn’t plan it that way—this stack naturally emerged as the project grew.

## Performance Notes

I didn’t start this project with performance benchmarks in mind, but once things stabilized, I tested it—and the results were better than I expected.

* sled is fast by default
* redb is very memory-efficient
* batch operations and zero-copy APIs ended up giving huge speedups

## What’s Next?

The next article in the series digs into procedural macros—the part that intimidated me the most when I started this project. I’ll walk through how I learned to parse Rust syntax trees, generate enums and trait impls, and structure macro code so it's maintainable.

## Conclusion

`netabase_store` grew out of my own frustration with juggling raw bytes and repetitive networking setup. As I kept improving it, it turned into a surprisingly robust, type-safe abstraction layer that works across multiple storage backends.

My hope is that by sharing my learning process—not just the outcomes—you'll get a clearer picture of how a Rust library like this evolves, and maybe feel inspired to experiment with similar ideas.

## References

[1]: https://libp2p.io/
[2]: https://en.wikipedia.org/wiki/Peer-to-peer
[3]: https://en.wikipedia.org/wiki/Kademlia
[4]: https://docs.libp2p.io/concepts/fundamentals/protocols/#distributed-hash-table-dht
[5]: https://docs.rs/libp2p-kad/latest/libp2p_kad/record/store/trait.RecordStore.html
[6]: https://serde.rs/
[7]: https://en.wikipedia.org/wiki/Database_index#Secondary_index
[8]: https://docs.rs/sled/
[9]: https://docs.rs/redb/
[10]: https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API
[11]: https://doc.rust-lang.org/reference/procedural-macros.html
[12]: https://doc.rust-lang.org/book/ch10-02-traits.html
[13]: https://docs.rs/bincode/
[14]: https://webassembly.org/
[15]: https://doc.rust-lang.org/book/ch19-06-macros.html#zero-cost-abstractions
[16]: https://cliffle.com/blog/rust-typestate/
[17]: https://blog.rust-lang.org/2022/10/28/gats-stabilization.html
[18]: https://doc.rust-lang.org/nomicon/phantom-data.html
[19]: https://doc.rust-lang.org/reference/conditional-compilation.html
[20]: https://www.youtube.com/watch?v=bSkpMdDe4g4
