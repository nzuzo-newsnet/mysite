#####
date = "2025-11-20"
author = "Nzuzo Magagula"
summary = "Deep dive into procedural macros and code generation - how to parse Rust syntax trees and generate type-safe database code at compile time"
thumbnail = "https://i.postimg.cc/pdKhS5Rk/netabase-store-architecture.png"
category = "Technical"
show_references = true

[[article_series]]
name = "Building netabase_store"
previous = "netabase_store/01-introduction-and-overview"
next = "netabase_store/03-backend-implementation-and-trait-design"
#####
# **Part 2 — Final Draft: The Complete Macro System**

### *Understanding the Derive Macros, AST Visitors, and Model Compilation Pipeline*

This section explains the two core macros that power Netabase: the **derive macros** (`#[derive(Model)]`) and the **declarative macro** (`netabase_definition_module!`). You will learn how they traverse the Rust syntax tree, build model metadata, and finally emit the compiled structures and traits used by the runtime.

Throughout this explanation, you’ll see that meta-logic (code that runs at *compile time* inside the macro) is fundamentally different from runtime logic (code that the macro *generates*, which your application uses at runtime). This distinction is subtle and confusing at first, so we will explicitly highlight it whenever it matters.

---

## **1. Overview of the Macro System**

The macros form a two-stage compilation system for your data model:

1.  **The Derive Macros (`#[derive(Model)]`, etc.):** These are procedural macros that operate on a single struct. They perform four major tasks:
    *   Parse the input AST.
    *   Visit each struct, field, and attribute.
    *   Extract model metadata (keys, fields, discriminants, etc.).
    *   Generate strongly typed Rust code (newtypes, traits, impls) from that metadata.

2.  **The Declarative Macro (`netabase_definition_module!`):** This macro acts as a linker. It takes a list of already-defined models and compiles them into a single, cohesive database definition module, generating discriminants, static descriptors, and the public database API.

Conceptually, this is similar to writing a mini-compiler for your data model layer. The macros do not merely automate small tasks—they generate an entire ecosystem of strongly typed structures and database-level identifiers.

This is the first area where distinguishing **meta-logic** vs **runtime logic** becomes essential:

*   **Meta-logic:** The macro parses your code, builds metadata, and generates new Rust code.
*   **Runtime logic:** What your program actually *does* with all that generated code.

Keeping those layers distinct makes the system intuitive once understood.

---

## **2. Stage 1: The Derive Macros and the Visitor Pattern**

### **The Visitor as a Census Worker**

Inside the derive macro, we use a custom **Visitor** to walk through the syntax tree. The easiest way to understand this is to imagine a **census worker** traveling through a neighborhood:

*   The **neighborhood** is your entire syntax tree.
*   Each **house** is a node (a struct, field, attribute, etc.).
*   The **census worker** is our Visitor struct.
*   The **paperwork** the census worker fills in is the metadata we collect (primary key, secondary key, field attributes, etc.).

The census worker visits every house, examines what’s inside, records facts, never *alters* anything, and hands the collected data to the “city database compiler” (the code generator). This analogy cleanly captures why the Visitor pattern is a perfect fit: it performs a structured, predictable walk over the tree, gathering the metadata needed to generate code.

### **Planned Migration: Using `darling` for Attribute Parsing**

Right now, attributes such as `#[primary_key]` and `#[secondary_index]` are parsed manually using raw `syn` inspection. This works, but nested conditionals grow messy as more attributes are supported.

We will migrate the macro parsing layer to **`darling`**, which will:
*   Completely remove deep nested conditionals.
*   Give us declarative attribute definitions.
*   Improve error messages.
*   Make the parsing code shorter and far more maintainable.

This is acknowledged in the source comments and is part of the intended future cleanup.

---

## **3. Extracting and Validating Model Metadata**

Once the Visitor has walked the AST, we end up with a `ModelMeta` structure containing the model's name, fields, primary key, secondary keys, and other attributes.

Several **basic assertions** occur here in the **meta-logic layer**:

### **Assertion A: Exactly one primary key must exist**
A model must declare exactly one primary key because it uniquely identifies entities, defines the storage layout, and determines which newtype must be generated. Allowing zero or multiple primary keys would break the contract that backends rely on.

### **Assertion B: All secondary keys must be newtypes**
Secondary keys cannot reuse primitive types like `String` or `u32`. They must be newtypes because they participate in the model’s `SecondaryKey` enum, and backends must be able to differentiate them by type for safety.

### **Assertion C: Discriminants are required**
Each model needs a unique discriminant so the backend can tag rows by model type, separate index namespaces, and prevent table collisions. The `DatabaseDefinition` macro will later generate the `ModelDiscriminant` enum using these.

### **Assertion D: Generated conversion traits must be valid**
`TryFrom<Enum>` implementations allow for ergonomic and safe extraction of typed keys from the model key enums, preventing fatal type-mixing mistakes in a strongly typed backend.

---

## **4. Generating Strongly Typed Code from a Model**

Once all metadata is validated, the derive macro generates the code. The output includes:
*   Newtypes for primary and secondary keys.
*   Implementations of `Borrow`, `From`, `TryFrom`, and `AsRef`.
*   The model’s key enums and `Descriptor` struct.
*   Backend-facing traits and helper methods for constructing keys.

This generated code is the “runtime logic” that your program and the backends will use.

### **Why We Implement `Borrow` (Redb Integration)**
A major reason we implement `Borrow<T>` on key types is because **Redb uses borrow-based key lookup types**. By supporting `Borrow`, we enable zero-copy lookup, direct byte comparison, and avoid runtime allocation. This is what enables very fast queries like `tree.get(&user_email)?;` where `user_email` might be a `&str`, but the stored key is a `UserEmailKey` newtype.

---

## **5. Stage 2: The `#[netabase_definition_module]` Attribute Macro**

### *The Compile-Time Database Schema Compiler*

While the derive macros operate at the **per-model** level, the `#[netabase_definition_module]` macro performs a different and equally critical role: **It transforms a module containing multiple model definitions into a single, cohesive database schema.**

Where derive macros are concerned with *struct → metadata → expansions*, this attribute macro handles database-wide model registration, discriminant generation, wrapper enums for type-safe queries, and the construction of the public database API surface. This is the macro that turns *a module of structs* into *a complete type-safe database schema*.

### **What This Macro Consumes and Generates**

The input is a **module containing models** annotated with the attribute:
```rust
#[netabase_definition_module(BlogDefinition, BlogKeys)]
pub mod blog {
    use netabase_store::{NetabaseModel, netabase};

    #[derive(NetabaseModel, Clone, Debug,
             bincode::Encode, bincode::Decode)]
    #[netabase(BlogDefinition)]
    pub struct User {
        #[primary_key]
        pub id: u64,
        pub username: String,
        #[secondary_key]
        pub email: String,
    }

    #[derive(NetabaseModel, Clone, Debug,
             bincode::Encode, bincode::Decode)]
    #[netabase(BlogDefinition)]
    pub struct Post {
        #[primary_key]
        pub id: u64,
        pub title: String,
        pub author_id: u64,
    }
}
```

The macro inspects the module AST, collects all models marked with `#[netabase(BlogDefinition)]`, and generates:

*   **`BlogDefinition` enum**: A wrapper enum containing all model types for type-safe storage and retrieval
*   **`BlogDefinitionDiscriminant` enum**: A `#[repr(u16)]` enum with variants for each model type, used as database-level namespace identifiers
*   **`BlogKeys` enum**: A wrapper enum containing all possible key types (primary and secondary) across all models
*   **`BlogDefinitionTables` struct** (Redb only): Static table definitions for compile-time table name validation
*   **Trait implementations**: Implementation of `NetabaseDefinitionTrait` providing metadata access and conversions

### **The Central Role of the Model Discriminant**
The macro generates the `BlogDefinitionDiscriminant` as a `#[repr(u16)]` enum where each variant corresponds to a model (e.g., `User`, `Post`). This discriminant acts as a *database-level namespace identifier*. Backends use it to:
- Separate tables by model type (e.g., "User" tree, "Post" tree)
- Create isolated secondary index namespaces
- Encode composite keys for secondary indexes
- Route typed queries to the correct storage region

This discriminant-based design enables completely generic backend code that can work with any schema defined by users.

### **Assertions Performed by This Macro**
This macro enforces **compile-time safety guarantees** through:
*   **Module structure validation**: The module must contain at least one model
*   **Type coherence checks**: All listed models must implement `NetabaseModelTrait<BlogDefinition>`
*   **Discriminant generation**: Each model automatically receives a unique discriminant value
*   **Key type safety**: Keys are correctly scoped to their parent definition through the wrapper enums

---

## **6. The Power of Macro-Generated Architectures**

Using macros to generate the model and database APIs is significantly more powerful than manual implementation or traditional runtime ORMs.

### **Macro-generated implementations give you:**

*   **✔ Strong Architectural Guarantees:** The compiler enforces that every model receives the same robust, type-safe interface.
*   **✔ Zero Boilerplate:** Dozens of types and impls are created instantly for each model.
*   **✔ No Drift:** Manually written impls become inconsistent over time; macro-generated ones never do.
*   **✔ Zero Runtime Cost:** All expansions are compile-time transformations.
*   **✔ Backend Portability:** Backends target a stable, auto-generated API, not user code.
*   **✔ Compile-Time Integrity Checks:** Mistakes like missing models or naming collisions are caught at compile time, not runtime.

### **A Complete Example of the Power**
Developers write:
```rust
#[netabase_definition_module(MyDb, MyDbKeys)]
mod database {
    use netabase_store::{NetabaseModel, netabase};

    #[derive(NetabaseModel, Clone, Debug,
             bincode::Encode, bincode::Decode)]
    #[netabase(MyDb)]
    pub struct User {
        #[primary_key]
        pub id: u64,
        #[secondary_key]
        pub email: String,
    }
}
```
...and the macro system collectively emits:
*   `UserPrimaryKey`, `UserEmailSecondaryKey` - typed key wrappers
*   `UserSecondaryKeys`, `UserKeys` - key enums for the User model
*   Key conversion impls and `Borrow` implementations for zero-copy lookups
*   `MyDb` enum - wrapper for all model types (contains `User` variant)
*   `MyDbDiscriminant` enum - contains `User` discriminant for namespace separation
*   `MyDbKeys` enum - wrapper for all key types across all models
*   `MyDbTables` struct (Redb) - static table definitions
*   Complete `NetabaseModelTrait<MyDb>` and `NetabaseDefinitionTrait` implementations
*   All the trait impls needed for backends to operate type-safely

Macros don't just reduce typing—they enforce a **correct domain architecture** at compile time.

---

## **7. Meta-Logic vs Runtime Logic — The Clear Separation**

This project is a prime example of why you must learn to separate meta and runtime thinking. The two layers never interact directly—only through generated code.

**Meta logic (macros):**
*   Runs during compilation.
*   Reads your code (derive) or a model list (declarative).
*   Generates more code.
*   Contains visitors, attribute parsing, assertions, and error messages.

**Runtime logic (generated code):**
*   Runs when your application executes.
*   Defines how models behave and how keys are constructed.
*   Enables backends to perform queries using descriptors and discriminants.

A useful mental model is:
> **Meta-logic is the compiler writing Rust code for you. Runtime logic is your program running that code.**

---

## **8. Closing Summary: The Full Compilation Pipeline**

Part 2 has shown that the Netabase macro system forms a complete, staged compilation pipeline:

1.  **AST Parsing & Visitation:** A **Visitor** (the "census worker") walks the AST of a single model annotated with `#[derive(NetabaseModel)]`.
2.  **Metadata Extraction & Validation:** Model metadata is extracted and compile-time assertions ensure strong correctness guarantees.
3.  **Per-Model Code Generation:** The derive macro emits runtime structures (key types, enums), trait implementations, and `Borrow` impls for a single model.
4.  **Database Schema Assembly:** The `#[netabase_definition_module]` attribute macro traverses the module, collects all models, and generates wrapper enums (`Definition`, `Discriminant`, `Keys`), static table definitions, and the complete schema trait implementations.
5.  **Backend Integration:** Backends use the generated schema through the trait APIs, leveraging `Borrow` for zero-copy access and discriminants for type-safe namespace separation.

This two-stage system, with its clear separation of meta-logic (compile-time code generation) and runtime logic (the generated code), provides a robust foundation for a type-safe, high-performance database layer. A future migration to `darling` will further improve the maintainability and clarity of the attribute parsing stage of this pipeline.
