# References for Building netabase_store Series

This document lists crates, concepts, and technical terms mentioned throughout the series that need links and references.

## Rust Crates

### Database Backends
- libp2p
- sled
- redb
- IndexedDB (WASM browser API)

### Serialization
- bincode

### Macro Development
- syn
- quote
- proc_macro
- darling (mentioned as future improvement)

### Async & WASM
- async_trait

### Configuration & Utilities
- typed-builder
- tempfile
- uuid
- ouroboros (mentioned for zero-copy self-referential structs)
- chrono

## Rust Language Concepts

### Procedural Macros
- Procedural macros
- Derive macros
- Attribute macros
- TokenStream
- DeriveInput
- parse_macro_input!
- quote! macro
- AST (Abstract Syntax Tree)

### Type System
- GATs (Generic Associated Types)
- Phantom types (PhantomData)
- Associated types
- Generic parameters
- Type-state pattern
- Zero-cost abstractions
- #[repr(u16)]

### Memory & Lifetimes
- Borrowing
- Lifetime management
- Zero-copy reads
- std::borrow::Borrow
- Interior mutability
- RefCell
- Arc

### Patterns & Design
- Visitor pattern
- Trait-based design
- Builder pattern
- RAII (Resource Acquisition Is Initialization)
- Single Responsibility Principle
- Data-Oriented Design (DOD)

### Compilation
- Feature flags
- Conditional compilation
- Semantic versioning

## Database & Storage Concepts

### Data Structures
- Key-value stores
- B-tree
- Secondary indexes
- Primary keys
- Composite keys

### Transactions
- ACID transactions
- Write-ahead log
- Read transactions
- Write transactions
- Transaction batching

### Performance
- SIMD optimizations
- Cache locality
- Batch operations
- Zero-copy operations
- Deserialization overhead

## Networking & Distributed Systems

- DHT (Distributed Hash Table)
- Kademlia
- RecordStore trait
- Peer-to-peer networking

## Cross-Platform Concepts

- WASM (WebAssembly)
- Native vs WASM compilation
- Browser storage APIs

## API Documentation Sites

- docs.rs
- crates.io
- Rust API Guidelines
- The Rustonomicon
- Rust by Example
- Rust Book
