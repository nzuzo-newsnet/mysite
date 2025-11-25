#####
date = "2025-11-20"
author = "Nzuzo Magagula"
summary = "Deep dive into procedural macros and code generation - how to parse Rust syntax trees and generate type-safe database code at compile time"
thumbnail = "https://i.postimg.cc/d1ZSWs9W/54a1b049-09d1-4d4b-82fd-2c620fbccc0c.jpg"
category = "Technical"
show_references = true

[[article_series]]
name = "Building netabase_store"
prev = "netabase_store/01-introduction-and-overview"
next = "netabase_store/03-backend-implementation-and-trait-design"

[[references]]
title = "Procedural Macros - The Rust Book"
url = "https://doc.rust-lang.org/book/ch19-06-macros.html"
description = "Official guide to macros in Rust"

[[references]]
title = "syn Crate Documentation"
url = "https://docs.rs/syn/latest/syn/"
description = "Library for parsing Rust syntax trees"

[[references]]
title = "quote Crate Documentation"
url = "https://docs.rs/quote/latest/quote/"
description = "Quasi-quoting for Rust code generation"

[[references]]
title = "Procedural Macros Reference"
url = "https://doc.rust-lang.org/reference/procedural-macros.html"
description = "Comprehensive reference for procedural macros"

[[references]]
title = "Abstract Syntax Tree (AST)"
url = "https://en.wikipedia.org/wiki/Abstract_syntax_tree"
description = "Understanding syntax trees in compiler design"
#####
# Part 2 — The Complete Macro System

### *Understanding the Derive Macros, AST Visitors, and Model Compilation Pipeline*

This section explains the two core macros that power Netabase: the **derive macros** (`#[derive(NetabaseModel)]`) and the **attribute macro** (`netabase_definition_module(Definition, DefinitionKeys`). You will learn how they traverse the [Rust syntax tree][1], build model metadata, and finally emit the compiled structures and traits used by the runtime.

Throughout this explanation, you’ll see that meta-logic (code that runs at *compile time* inside the macro) is fundamentally different from runtime logic (code that the macro *generates*, which your application uses at runtime). This distinction is subtle and confusing at first, so we will explicitly highlight it whenever it matters.

---

## **1. Overview of the Macro System**

The macros form a two-stage compilation system for your data model:

1.  **The Derive Macros (`#[derive(Model)]`, etc.):** These are [procedural macros][2] that operate on a single struct. They perform four major tasks:
    *   Parse the input [AST][1].
    *   Visit each struct, field, and attribute.
    *   Extract model metadata (keys, fields, discriminants, etc.).
    *   Generate strongly typed Rust code (newtypes, [traits][3], impls) from that metadata.

2.  **The Attribute Macro (`netabase_definition_module(Definition, DefinitionKeys)`):** This macro acts as a linker. It takes a list of already-defined models and compiles them into a single, cohesive database definition module, generating discriminants, static descriptors, and the public database API.

Conceptually, this is similar to writing a mini-compiler for your data model layer. The macros do not merely automate small tasks—they generate an entire ecosystem of strongly typed structures and database-level identifiers.

This is the first area where distinguishing **meta-logic** vs **runtime logic** becomes essential:

*   **Meta-logic:** The macro parses your code, builds metadata, and generates new Rust code.
*   **Runtime logic:** What your program actually *does* with all that generated code.

Keeping those layers distinct makes the system intuitive once understood.

Of course! This is a fantastic piece of technical writing. Let's complete the "Macro crate structure" section by chronicling the evolution from a monolithic function to the separated Visitor/Generator architecture, using the code from your `lib.rs` as the final result of this journey.

Here's the new, expanded section:

---

## **Macro Crate Structure: An Evolution from Chaos to Clarity**

Macros are super powerful because they are super flexible. When you decide to use macros for code generation, there is so much that is possible, it actually becomes difficult to manage.
When I got started, I could not find many defined structures and common practices for a macro library that needed to do what I needed my macros to do. Quite frankly, I was not sure *what exactly* I wanted from my macros and this experience was my favorite trial and error process.

### **1. The Monolithic Function: When Everything Lives Together**

I started by trying to define macros and their functionality in one single function, which quickly became a headache:

```rust
// First attempt: Everything in one giant function
#[proc_macro_derive(NetabaseModel, attributes(primary_key, secondary_key, link))]
pub fn netabase_model_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    // Parse attributes manually
    let mut primary_key = None;
    let mut secondary_keys = Vec::new();
    
    if let syn::Data::Struct(data) = &input.data {
        for field in &data.fields {
            for attr in &field.attrs {
                if attr.path().is_ident("primary_key") {
                    if primary_key.is_some() {
                        panic!("Multiple primary keys found");
                    }
                    primary_key = Some(field.ident.as_ref().unwrap());
                } else if attr.path().is_ident("secondary_key") {
                    secondary_keys.push(field.ident.as_ref().unwrap());
                }
            }
        }
    }
    
    let primary_key = primary_key.expect("No primary key found");
    
    // Generate primary key newtype
    let primary_key_name = format!("{}PrimaryKey", input.ident);
    let primary_key_ident = Ident::new(&primary_key_name, input.ident.span());
    
    // Generate secondary key types and enum
    let secondary_key_types = secondary_keys.iter().map(|key| {
        let type_name = format!("{}{}SecondaryKey", input.ident, key);
        Ident::new(&type_name, input.ident.span())
    });
    
    let secondary_key_variants = secondary_keys.iter().map(|key| {
        let variant_name = format!("{}", key);
        Ident::new(&variant_name, input.ident.span())
    });
    
    // Generate trait implementation
    let model_name = &input.ident;
    
    // ... and 100+ more lines of quote!{} spaghetti
}
```

You can see how much of a pain it would be to update the derive macro for every single feature I wanted added. The problem was pretty obvious: it is difficult to see what is not working when everything is in the same function. DUH!
That is like the first rule of good design. The problems were immediate. See, I could not really tell what an error in the expansion meant because it is all evaluated as a clump. Error messages from the compiler just noted that the macro was the problem, but i could not see what was wrong until i walked through the logic piece by piece.
This is especially annoying because if your macros are wrong enough, the code wont even expand at all!

### **2. The Helper Function Breakout: Separating Concerns**

My first improvement was to extract helper functions for different generation tasks. This made the code more readable but didn't solve the fundamental problem:

```rust
fn generate_primary_key(struct_name: &Ident, field: &Field) -> TokenStream2 {
    let key_name = format!("{}PrimaryKey", struct_name);
    let key_ident = Ident::new(&key_name, struct_name.span());
    let field_type = &field.ty;
    
    quote! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, bincode::Encode, bincode::Decode)]
        pub struct #key_ident(pub #field_type);
    }
}

fn generate_secondary_keys(struct_name: &Ident, fields: &[Field]) -> Vec<TokenStream2> {
    fields.iter().filter_map(|field| {
        if has_attribute(field, "secondary_key") {
            let field_name = field.ident.as_ref().unwrap();
            let key_name = format!("{}{}SecondaryKey", struct_name, field_name);
            let key_ident = Ident::new(&key_name, struct_name.span());
            let field_type = &field.ty;
            
            Some(quote! {
                #[derive(Debug, Clone, PartialEq, Eq, Hash, bincode::Encode, bincode::Decode)]
                pub struct #key_ident(pub #field_type);
            })
        } else {
            None
        }
    }).collect()
}

// The main function became cleaner but still mixed parsing and generation
#[proc_macro_derive(NetabaseModel, attributes(primary_key, secondary_key, link))]
pub fn netabase_model_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let primary_key_field = find_primary_key(&input).expect("No primary key found");
    let secondary_key_fields = find_secondary_keys(&input);
    
    let primary_key = generate_primary_key(&input.ident, &primary_key_field);
    let secondary_keys = generate_secondary_keys(&input.ident, &secondary_key_fields);
    let keys_enum = generate_keys_enum(&input.ident, &secondary_key_fields);
    let trait_impl = generate_trait_impl(&input.ident, &primary_key_field, &secondary_key_fields);
    
    quote! {
        #primary_key
        #(#secondary_keys)*
        #keys_enum
        #trait_impl
    }.into()
}
```

This was better, but I still couldn't easily test individual components or get good error messages. When something went wrong, I had to guess which helper function was causing the issue.

### **3. Adding Error Messages: Finding the Problematic Behavior**

The real breakthrough came when I started adding meaningful error messages. Instead of panicking with "Multiple primary keys found", I needed to tell users exactly what went wrong and where:

```rust
fn validate_model(input: &DeriveInput) -> Result<(), syn::Error> {
    let mut primary_keys = Vec::new();
    let mut errors = Vec::new();
    
    if let syn::Data::Struct(data) = &input.data {
        for field in &data.fields {
            for attr in &field.attrs {
                if attr.path().is_ident("primary_key") {
                    primary_keys.push(field);
                }
            }
        }
    }
    
    if primary_keys.is_empty() {
        errors.push(syn::Error::new_spanned(
            input,
            "Model must have exactly one #[primary_key] field"
        ));
    } else if primary_keys.len() > 1 {
        for key in primary_keys {
            errors.push(syn::Error::new_spanned(
                key,
                "Multiple #[primary_key] fields found. Only one primary key is allowed per model."
            ));
        }
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.into_iter().reduce(|mut a, b| {
            a.combine(b);
            a
        }).unwrap())
    }
}
```

This approach was much better—users got clear error messages—but the code was still hard to maintain. The validation logic was scattered across multiple functions, and adding new features meant touching every part of the codebase.

### **4. The Visitor/Generator Separation: The Final Architecture**

The final breakthrough came when I realized I needed to separate the **parsing/visiting** logic from the **code generation** logic completely. This led to the current architecture:

```rust
// Clean separation in the main derive function
#[proc_macro_derive(NetabaseModel, attributes(primary_key, secondary_key, link))]
pub fn netabase_model_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    // VISITOR: Extract metadata from AST
    let mut visitor = ModelVisitor::default();
    visitor.visit_derive_input(&input);
    
    // GENERATORS: Create code from metadata
    let (p, sl, s, k) = visitor.generate_keys();
    let trait_impl = visitor.generate_model_trait_impl();
    let borrow_impls = visitor.generate_borrow_impls();
    let extension_traits = visitor.generate_key_extension_traits();

    quote! {
        #p        // Primary key
        #(#sl)*   // Secondary key list  
        #s        // Secondary keys enum
        #k        // Combined keys enum
        #(#trait_impl)*
        #(#borrow_impls)*
        #(#extension_traits)*
    }.into()
}
```

This architecture provides several key benefits:

1. **Testability**: I can test the `ModelVisitor` independently to ensure it extracts the right metadata
2. **Maintainability**: The generation logic is organized by concern (keys, traits, borrow impls, etc.)
3. **Error Isolation**: When something goes wrong, I know exactly which component to check
4. **Extensibility**: Adding new features means either extending the visitor or adding a new generator

The same pattern applies to the `netabase_definition_module` macro:

```rust
#[proc_macro_attribute]
pub fn netabase_definition_module(name: TokenStream, input: TokenStream) -> TokenStream {
    let mut def_module = parse_macro_input!(input as ItemMod);
    
    // VISITOR: Extract module structure and model information
    let mut visitor = DefinitionsVisitor::default();
    visitor.visit_item_mod(&def_module);
    
    // GENERATORS: Create definition enums, trait impls, etc.
    let (defin, def_key) = visitor.generate_definitions(definition, definition_key);
    let tables_struct = generators::table_definitions::generate_tables_struct(&visitor.modules, definition);
    let trait_impls = visitor.generate_definition_trait_impls(definition, definition_key, &tables_name);
    
    // ... more generators
}
```

This separation of concerns is what makes the macro system maintainable. The visitors are responsible for understanding the structure of the code, while the generators are responsible for emitting new code based on that understanding. Each can evolve independently, and issues can be pinpointed quickly.

---

## **2. Stage 1: The Derive Macros and the Visitor Pattern**

### **A little bit about Syn**

#### **How [`syn`][5] Enables AST Climbing: The Macro Author's Toolkit**

The [`syn`][5] crate is the foundation that makes [procedural macros][2] possible in Rust. It provides the tools to parse, traverse, and understand Rust code at compile time. Think of [`syn`][5] as giving you a detailed map and climbing gear for navigating Rust's [Abstract Syntax Tree][1] ([AST][1]).

### **What is an Abstract Syntax Tree?**

An AST is a tree representation of your code's structure. When you write:
```rust
#[derive(NetabaseModel)]
pub struct User {
    #[primary_key]
    pub id: u64,
    #[secondary_key] 
    pub email: String,
}
```

The Rust compiler first parses this into an AST that might look conceptually like:
```
DeriveInput
├── attributes: ["NetabaseModel"]
├── vis: "pub"  
├── ident: "User"
└── Data::Struct
    └── Fields::Named
        ├── Field
        │   ├── attributes: ["primary_key"]
        │   ├── vis: "pub"
        │   ├── ident: "id"
        │   └── ty: "u64"
        └── Field
            ├── attributes: ["secondary_key"]
            ├── vis: "pub" 
            ├── ident: "email"
            └── ty: "String"
```

##### **How `syn` Parses TokenStream into AST**

When a procedural macro runs, it receives a `TokenStream` - a flat sequence of tokens without structural understanding. `syn` transforms this into a structured AST:

```rust
// TokenStream (flat, hard to work with):
// #[ derive ( NetabaseModel ) ] pub struct User { # [ primary_key ] pub id : u64 , ... }

// syn parses this into a structured DeriveInput (easy to work with):
let input = parse_macro_input!(input as DeriveInput);
// Now we have a structured object with fields, attributes, etc.
```

##### **The [Visitor Pattern][6]: Your AST Climbing Gear**

The [`syn::visit::Visit`][7] [trait][3] is what enables the "census worker" pattern described earlier. It provides methods for every type of node in the [AST][1], allowing you to climb through the tree systematically:

```rust
impl<'a> Visit<'a> for ModelVisitor<'a> {
    fn visit_derive_input(&mut self, node: &'a DeriveInput) {
        // We've reached a struct/enum definition - our main target
        println!("Visiting struct: {}", node.ident);
        syn::visit::visit_derive_input(self, node); // Continue climbing
    }
    
    fn visit_field(&mut self, node: &'a Field) {
        // We're now looking at an individual field
        println!("Found field: {:?}", node.ident);
        syn::visit::visit_field(self, node); // Continue to attributes, type, etc.
    }
    
    fn visit_attribute(&mut self, node: &'a Attribute) {
        // We're examining an attribute like #[primary_key]
        if node.path().is_ident("primary_key") {
            println!("Found primary key attribute!");
        }
        syn::visit::visit_attribute(self, node);
    }
}
```

##### **Manual AST Navigation vs. [Visitor Pattern][6]**

[`syn`][5] gives you two ways to climb the [AST][1]:

**1. Manual Navigation (Direct Access):**
```rust
fn visit_derive_input(&mut self, i: &'a DeriveInput) {
    // Directly access what we need
    self.name = Some(&i.ident);                    // Struct name
    self.definitions = Self::find_definitions(i);  // Custom attributes
    
    if let syn::Data::Struct(data) = &i.data {     // Drill into struct body
        for field in &data.fields {                // Examine each field
            // Process field attributes, types, etc.
        }
    }
}
```

**2. Automatic Traversal (Visitor Pattern):**
```rust
fn visit_field(&mut self, field: &'a Field) {
    // This gets called automatically for every field
    // The visitor pattern walks the entire tree for us
}
```

In Netabase, we use a hybrid approach: we implement [`Visit`][7] but override specific methods to collect exactly what we need, then use manual navigation within those methods for precise extraction.

#### **Real-World AST Climbing in Netabase**

Let's trace through what happens when `syn` processes our example:

```rust
#[derive(NetabaseModel)]
#[netabase(BlogDefinition)] 
pub struct User {
    #[primary_key]
    pub id: u64,
    #[secondary_key]
    pub email: String,
}
```

1. **[TokenStream][8] → DeriveInput**: [`syn`][5] parses the raw tokens into a structured `DeriveInput`
2. **[Visitor][6] Entry**: Our `visit_derive_input` method is called with the complete struct
3. **Metadata Extraction**: We manually extract:
   - Struct name: `User`
   - Definition: `BlogDefinition` (from `#[netabase]` attribute)
   - Fields: We examine each field and its attributes
4. **Key Discovery**: For each field, we check attributes to identify `#[primary_key]` and `#[secondary_key]`
5. **Type Analysis**: We examine field types (`u64`, `String`) to generate appropriate newtypes

### **Why This Matters for Macro Authors**

Understanding `syn` and AST traversal is crucial because:

- **Precise Targeting**: You only generate code for the specific structures you care about
- **Context Awareness**: You understand the relationships between attributes, fields, and types
- **Error Prevention**: You can validate that the input makes sense before generating code
- **Flexibility**: You can support complex Rust features like [generics][9], [lifetimes][10], and complex types

### **The Compilation Pipeline with `syn`**

The complete flow looks like:
```
Rust Source Code
    → [TokenStream][8] (raw tokens)
    → [syn][5]::DeriveInput (structured [AST][1])
    → ModelVisitor (extract metadata)
    → Code Generators (emit new code using [quote][11])
    → [TokenStream][8] (generated code)
    → Expanded Rust Code
```

This pipeline is what enables Netabase to understand your data model at compile time and generate the appropriate type-safe database code. Without [`syn`][5]'s robust parsing and visitation capabilities, we'd be stuck trying to make sense of raw token streams - a much more error-prone and limited approach.

The [Visitor pattern][6], combined with [`syn`][5]'s comprehensive [AST][1] types, provides the solid foundation that makes complex macro systems like Netabase possible. It's the difference between trying to understand a book by looking at individual letters versus reading complete sentences and paragraphs.
### **The Visitor as a Census Worker**

Inside the derive macro, we use a custom **Visitor** to walk through the syntax tree. The easiest way to understand this is to imagine a **census worker** traveling through a neighborhood:

*   The **neighborhood** is your entire syntax tree.
*   Each **house** is a node (a struct, field, attribute, etc.).
*   The **census worker** is our Visitor struct.
*   The **paperwork** the census worker fills in is the metadata we collect (primary key, secondary key, field attributes, etc.).

The census worker visits every house, examines what’s inside, records facts, never *alters* anything, and hands the collected data to the “city database compiler” (the code generator). This analogy cleanly captures why the Visitor pattern is a perfect fit: it performs a structured, predictable walk over the tree, gathering the metadata needed to generate code.


```rust
use [syn][5]::{Ident, Path, Token, punctuated::Punctuated, visit::Visit};

use crate::{
    item_info::netabase_model::{ModelKeyInfo, ModelLinkInfo},
    util::extract_fields,
};

// THE CENSUS WORKER'S PAPERWORK: This struct holds all the metadata collected
// during the [AST][1] traversal. Each field represents a different type of information
// that our "census worker" (the [visitor pattern][6]) gathers from the syntax tree neighborhood.
#[derive(Default)]
pub struct ModelVisitor<'ast> {
    pub name: Option<&'ast Ident>,           // The "house address" - which struct we're visiting
    pub key: Option<ModelKeyInfo<'ast>>,     // Primary and secondary key information found
    pub links: Vec<ModelLinkInfo<'ast>>,     // Foreign key relationships (future links)
    pub definitions: Vec<Path>,              // Which database definition this model belongs to
    // Generics support removed - not yet implemented
    // pub generics: Option<&'ast Generics>, // Future: support for generic models
}

// THE CENSUS WORKER'S ROUTE: This implementation defines exactly what our visitor
// does when it encounters different nodes in the syntax tree neighborhood.
impl<'a> Visit<'a> for ModelVisitor<'a> {
    // MAIN HOUSE VISIT: This is called when we visit a struct definition (the main "house")
    fn visit_derive_input(&mut self, i: &'a syn::DeriveInput) {
        // RECORD THE ADDRESS: Note which struct we're examining
        self.name = Some(&i.ident);
        
        // Generics support removed - not yet implemented
        // self.generics = Some(&i.generics);
        
        // EXAMINE THE RESIDENTS: Look at all fields in the struct and identify keys
        // This is like checking who lives in the house and their roles (primary/secondary keys)
        self.key = match ModelKeyInfo::find_keys(extract_fields(i)) {
            Ok(k) => Some(k),
            Err(e) => panic!("Error parsing Model: {e}"),
        };
        
        // CHECK HOUSE AFFILIATIONS: Find which database definition this model belongs to
        // This is like noting which neighborhood district the house is part of
        self.definitions = Self::find_definitions(i);
        
        // MAP CONNECTIONS TO OTHER HOUSES: Find foreign key relationships
        // This is like noting which other houses this one is connected to
        self.links = ModelLinkInfo::find_link(extract_fields(i)).collect();
    }
}

// THE CENSUS WORKER'S SPECIALIZED TOOLS: Helper methods that assist in the visitation process
impl<'a> ModelVisitor<'a> {
    // SPECIALIZED DATA COLLECTION: This method specifically looks for the `#[netabase]` 
    // attribute to determine which database definition the model belongs to.
    // Think of this as checking the house's official registration documents.
    pub fn find_definitions(input: &'a syn::DeriveInput) -> Vec<syn::Path> {
        // LOOK FOR THE OFFICIAL STAMP: Find the #[netabase(...)] attribute
        let attr = input.attrs.iter().find(|a| a.path().is_ident("netabase"));
        
        // READ THE REGISTRATION DETAILS: Parse what's inside the attribute
        if let Some(att) = attr
            && let Ok(list) = att.meta.require_list()
        {
            match list
                .parse_args_with(Punctuated::<syn::Path, Token![,]>::parse_terminated)
                .map_err(|e| e.into_compile_error())
            {
                Ok(r) => r.into_iter().collect(),  // Successfully read the definition names
                Err(_) => vec![],                  // Couldn't parse, return empty
            }
        } else {
            vec![]  // No netabase attribute found
        }
    }
}
```

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

## **6. From AST to Expanded Code: The Structural Transformation**

Now that we understand how `syn` enables AST traversal, let's see the core structural transformation from user input to generated types. This is where the meta-logic (macro execution) produces the runtime data structures that your application actually uses.

### **The Input → Output Transformation**

Let's trace a complete example through the macro system:

**User Input (9 lines):**
```rust
#[netabase_definition_module(ExampleDefs, ExampleDefKeys)]
pub mod definitions {
    use netabase_store::{NetabaseModel, netabase};

    #[derive(NetabaseModel, bincode::Encode, bincode::Decode, Clone, Debug)]
    #[netabase(ExampleDefs)]
    pub struct User {
        #[primary_key]
        pub name: String,
        pub age: u8,
        #[secondary_key]
        pub email: String,
    }
}
```

**Generated Core Structures (Key Types):**
```rust
// PRIMARY KEY NEWTYPE: Generated from #[primary_key] field
pub struct UserPrimaryKey(pub String);

// SECONDARY KEY NEWTYPE: Generated from #[secondary_key] field  
pub struct UserEmailSecondaryKey(pub String);

// SECONDARY KEYS ENUM: Unifies all secondary keys for this model
pub enum UserSecondaryKeys {
    Email(UserEmailSecondaryKey),
}

// COMBINED KEYS ENUM: Unifies primary and secondary keys
pub enum UserKey {
    Primary(UserPrimaryKey),
    Secondary(UserSecondaryKeys),
}
```

**Generated Database Schema Structures:**
```rust
// DATABASE DEFINITION ENUM: Can hold any model in the schema
pub enum ExampleDefs {
    User(User),
    // Additional models would be added here as the schema grows
}

// DATABASE KEYS ENUM: Can hold any key from any model in the schema
pub enum ExampleDefKeys {
    UserKey(UserKey),
    // PostKey(PostKey),      // Additional model keys would appear here
    // CommentKey(CommentKey),
}

// DISCRIMINANT TYPES: For type-safe model identification
pub enum ExampleDefsDiscriminant { 
    User 
}

pub enum ExampleDefKeysDiscriminant { 
    UserKey 
}
```

### **The Structural Expansion Pipeline**

Let's break down how each structure gets generated:

#### **1. Key Newtype Generation**
The visitor detects `#[primary_key]` and `#[secondary_key]` attributes and generates corresponding newtypes:

```rust
// Input field:
#[primary_key]
pub name: String,           // Field name: "name", type: String

// Generated newtype:
pub struct UserPrimaryKey(pub String);  // Name: User + PrimaryKey, wraps String

// Input field:  
#[secondary_key]
pub email: String,          // Field name: "email", type: String

// Generated newtype:
pub struct UserEmailSecondaryKey(pub String);  // Name: User + Email + SecondaryKey
```

**Why this matters:** These newtypes provide compile-time type safety - you can't accidentally use a `UserPrimaryKey` where a `PostPrimaryKey` is expected, even though both might wrap `String`.

#### **2. Enum Generation for Unified Access**
The generator creates enums that unify all keys for ergonomic usage:

```rust
// Generated from all #[secondary_key] fields
pub enum UserSecondaryKeys {
    Email(UserEmailSecondaryKey),  // One variant per secondary key
    // Age(UserAgeSecondaryKey),   // Additional keys would appear here
}

// Generated to combine primary and secondary keys  
pub enum UserKey {
    Primary(UserPrimaryKey),       // Primary key variant
    Secondary(UserSecondaryKeys),  // All secondary keys variant
}
```

**Why this matters:** These enums allow you to work with any key type through a unified interface while maintaining full type information.

#### **3. Database Schema Assembly**
The `netabase_definition_module` macro links all models together into a cohesive schema:

```rust
// Generated definition enum (holds any model in the module)
pub enum ExampleDefs {
    User(User),           // Your original User struct
    // Post(Post),       // Additional models would be variants
    // Comment(Comment),
}

// Generated keys enum (holds any key from any model)  
pub enum ExampleDefKeys {
    UserKey(UserKey),     // All keys from User model
    // PostKey(PostKey),   // All keys from Post model  
    // CommentKey(CommentKey),
}
```

**Why this matters:** This provides the unified interface that backends use to work with entire schemas generically, while maintaining type safety across different models.

#### **4. Discriminant Types for Runtime Identification**
The macro generates companion enums for efficient model identification:

```rust
// Simple enums used for efficient type tagging
pub enum ExampleDefsDiscriminant { User }
pub enum ExampleDefKeysDiscriminant { UserKey }
```

**Why this matters:** These discriminants enable backends to efficiently route operations to the correct storage without runtime type checking.

### **The Visitor → Generator Flow**

This structural transformation follows a clear pipeline:

1. **Visitor Examines AST**: The `ModelVisitor` finds `#[primary_key]` on `name: String` and `#[secondary_key]` on `email: String`
2. **Metadata Extraction**: Visitor records that `User` has primary key `name` (type `String`) and secondary key `email` (type `String`)  
3. **Generator Creates Types**: 
   - `UserPrimaryKey` newtype wrapping `String`
   - `UserEmailSecondaryKey` newtype wrapping `String`
   - `UserSecondaryKeys` enum with `Email` variant
   - `UserKey` enum combining primary and secondary
4. **Schema Assembly**: `DefinitionsVisitor` collects all models and generates the `ExampleDefs` and `ExampleDefKeys` enums

### **The Power of Structural Generation**

This expansion demonstrates why macros excel at database layers:

- **9 lines of user input** generate **8 distinct type definitions** that form a complete, type-safe database interface
- Each generated structure has a specific role in the overall architecture
- The naming follows consistent patterns (`{Model}PrimaryKey`, `{Model}{Field}SecondaryKey`, etc.)
- All type relationships are preserved and enforced by the Rust compiler

The macro system transforms your simple struct definition into a rich type ecosystem that provides compile-time guarantees for database operations, eliminating whole classes of runtime errors while maintaining excellent performance.

In addition to just generating type, you can add trait definitions, extra modules or any other code that you might need to create a functional system.

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

1.  **[AST][1] Parsing & Visitation:** A **[Visitor][6]** (the "census worker") walks the [AST][1] of a single model annotated with `#[derive(NetabaseModel)]`.
2.  **Metadata Extraction & Validation:** Model metadata is extracted and compile-time assertions ensure strong correctness guarantees.
3.  **Per-Model Code Generation:** The derive macro emits runtime structures (key types, [enums][12]), [trait][3] implementations, and [`Borrow`][13] impls for a single model.
4.  **Database Schema Assembly:** The `#[netabase_definition_module]` attribute macro traverses the module, collects all models, and generates wrapper [enums][12] (`Definition`, `Discriminant`, `Keys`), static table definitions, and the complete schema [trait][3] implementations.
5.  **Backend Integration:** Backends use the generated schema through the [trait][3] APIs, leveraging [`Borrow`][13] for zero-copy access and discriminants for type-safe namespace separation.

This two-stage system, with its clear separation of meta-logic (compile-time code generation) and runtime logic (the generated code), provides a robust foundation for a type-safe, high-performance database layer. A future migration to [`darling`][14] will further improve the maintainability and clarity of the attribute parsing stage of this pipeline.

## References

[1]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
[2]: https://doc.rust-lang.org/reference/procedural-macros.html
[3]: https://doc.rust-lang.org/book/ch10-02-traits.html
[4]: https://docs.rs/bincode/
[5]: https://docs.rs/syn/
[6]: https://en.wikipedia.org/wiki/Visitor_pattern
[7]: https://docs.rs/syn/latest/syn/visit/trait.Visit.html
[8]: https://doc.rust-lang.org/proc_macro/struct.TokenStream.html
[9]: https://doc.rust-lang.org/book/ch10-01-syntax.html
[10]: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
[11]: https://docs.rs/quote/
[12]: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
[13]: https://doc.rust-lang.org/std/borrow/trait.Borrow.html
[14]: https://docs.rs/darling/
