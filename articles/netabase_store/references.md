# References for Building netabase_store Series

This document lists crates, concepts, and technical terms mentioned throughout the series that need links and references.

## Rust Crates

### Database Backends
- libp2p <!-- TODO: Add reference to https://docs.rs/libp2p -->
- sled <!-- TODO: Add reference to https://docs.rs/sled -->
- redb <!-- TODO: Add reference to https://docs.rs/redb -->
- IndexedDB (WASM browser API) <!-- TODO: Add reference to MDN docs -->

### Serialization
- bincode <!-- TODO: Add reference to https://docs.rs/bincode -->

### Macro Development
- syn <!-- TODO: Add reference to https://docs.rs/syn -->
- quote <!-- TODO: Add reference to https://docs.rs/quote -->
- proc_macro <!-- TODO: Add reference to Rust docs -->
- darling (mentioned as future improvement) <!-- TODO: Add reference to https://docs.rs/darling -->

### Async & WASM
- async_trait <!-- TODO: Add reference to https://docs.rs/async-trait -->

### Configuration & Utilities
- typed-builder <!-- TODO: Add reference to https://docs.rs/typed-builder -->
- tempfile <!-- TODO: Add reference to https://docs.rs/tempfile -->
- uuid <!-- TODO: Add reference to https://docs.rs/uuid -->
- ouroboros (mentioned for zero-copy self-referential structs) <!-- TODO: Add reference to https://docs.rs/ouroboros -->
- chrono <!-- TODO: Add reference to https://docs.rs/chrono -->

## Rust Language Concepts

### Procedural Macros
- Procedural macros <!-- TODO: Add reference to Rust Book chapter on macros -->
- Derive macros <!-- TODO: Add reference to Rust Reference -->
- Attribute macros <!-- TODO: Add reference to Rust Reference -->
- TokenStream <!-- TODO: Add reference to proc_macro docs -->
- DeriveInput <!-- TODO: Add reference to syn docs -->
- parse_macro_input! <!-- TODO: Add reference to syn docs -->
- quote! macro <!-- TODO: Add reference to quote docs -->
- AST (Abstract Syntax Tree) <!-- TODO: Add reference to Wikipedia or Rust compiler docs -->

### Type System
- GATs (Generic Associated Types) <!-- TODO: Add reference to Rust RFC or blog post -->
- Phantom types (PhantomData) <!-- TODO: Add reference to Rust std docs -->
- Associated types <!-- TODO: Add reference to Rust Book -->
- Generic parameters <!-- TODO: Add reference to Rust Book -->
- Type-state pattern <!-- TODO: Add reference to blog post or pattern documentation -->
- Zero-cost abstractions <!-- TODO: Add reference to Rust philosophy docs -->
- #[repr(u16)] <!-- TODO: Add reference to Rust Reference on type layout -->

### Memory & Lifetimes
- Borrowing <!-- TODO: Add reference to Rust Book chapter -->
- Lifetime management <!-- TODO: Add reference to Rust Book chapter -->
- Zero-copy reads <!-- TODO: Add reference to relevant article or docs -->
- std::borrow::Borrow <!-- TODO: Add reference to Rust std docs -->
- Interior mutability <!-- TODO: Add reference to Rust Book or std docs -->
- RefCell <!-- TODO: Add reference to Rust std docs -->
- Arc <!-- TODO: Add reference to Rust std docs -->

### Patterns & Design
- Visitor pattern <!-- TODO: Add reference to design patterns book or Rust patterns -->
- Trait-based design <!-- TODO: Add reference to Rust Book or patterns -->
- Builder pattern <!-- TODO: Add reference to Rust design patterns -->
- RAII (Resource Acquisition Is Initialization) <!-- TODO: Add reference to Wikipedia or C++ docs -->
- Single Responsibility Principle <!-- TODO: Add reference to SOLID principles -->
- Data-Oriented Design (DOD) <!-- TODO: Add reference to blog post or book -->

### Compilation
- Feature flags <!-- TODO: Add reference to Cargo Book -->
- Conditional compilation <!-- TODO: Add reference to Rust Reference -->
- Semantic versioning <!-- TODO: Add reference to semver.org -->

## Database & Storage Concepts

### Data Structures
- Key-value stores <!-- TODO: Add reference to database fundamentals article -->
- B-tree <!-- TODO: Add reference to Wikipedia or data structures guide -->
- Secondary indexes <!-- TODO: Add reference to database indexing article -->
- Primary keys <!-- TODO: Add reference to database design fundamentals -->
- Composite keys <!-- TODO: Add reference to database design article -->

### Transactions
- ACID transactions <!-- TODO: Add reference to Wikipedia or database article -->
- Write-ahead log <!-- TODO: Add reference to database internals article -->
- Read transactions <!-- TODO: Add reference to transaction processing docs -->
- Write transactions <!-- TODO: Add reference to transaction processing docs -->
- Transaction batching <!-- TODO: Add reference to performance optimization article -->

### Performance
- SIMD optimizations <!-- TODO: Add reference to SIMD tutorial or Intel docs -->
- Cache locality <!-- TODO: Add reference to performance optimization article -->
- Batch operations <!-- TODO: Add reference to database performance article -->
- Zero-copy operations <!-- TODO: Add reference to systems programming article -->
- Deserialization overhead <!-- TODO: Add reference to serialization performance article -->

## Networking & Distributed Systems

- DHT (Distributed Hash Table) <!-- TODO: Add reference to Wikipedia or distributed systems article -->
- Kademlia <!-- TODO: Add reference to Kademlia paper or libp2p docs -->
- RecordStore trait <!-- TODO: Add reference to libp2p documentation -->
- Peer-to-peer networking <!-- TODO: Add reference to P2P fundamentals article -->

## Cross-Platform Concepts

- WASM (WebAssembly) <!-- TODO: Add reference to WebAssembly.org -->
- Native vs WASM compilation <!-- TODO: Add reference to Rust WASM book -->
- Browser storage APIs <!-- TODO: Add reference to MDN Web APIs -->

## API Documentation Sites

- docs.rs <!-- TODO: Add link to https://docs.rs -->
- crates.io <!-- TODO: Add link to https://crates.io -->
- Rust API Guidelines <!-- TODO: Add link to Rust API guidelines -->
- The Rustonomicon <!-- TODO: Add link to Rustonomicon -->
- Rust by Example <!-- TODO: Add link to Rust by Example -->
- Rust Book <!-- TODO: Add link to The Rust Programming Language book -->
