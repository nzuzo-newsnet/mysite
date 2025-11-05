# Advanced Markdown Parser

A sophisticated markdown parsing library designed to extract rich metadata, create knowledge graphs, and enable interactive experiences from markdown documents.

## Vision

This parser goes beyond traditional markdown rendering to extract structured information that can be used for:

- **AI-powered features**: Generate summaries, answer questions, create interactive tutorials
- **Knowledge graphs**: Build typed relationships between articles, concepts, and resources
- **Interactive experiences**: Automatically embed terminals, demos, and code playgrounds
- **Rich metadata extraction**: References, resources, related content, embedded sections

## Architecture

### Core Components

1. **Metadata Extractor**
   - TOML frontmatter parsing
   - Inline metadata annotations
   - Section-level metadata

2. **Content Analyzer**
   - AST (Abstract Syntax Tree) generation
   - Semantic analysis of content structure
   - Code block detection and classification

3. **Reference Manager**
   - Bibliography extraction
   - Cross-document references
   - External resource tracking
   - Citation formatting

4. **Graph Builder**
   - Entity extraction (concepts, terms, people)
   - Relationship mapping with typed edges
   - Hierarchical structure representation
   - Series and grouping relationships

5. **Interactive Elements**
   - Code playground detection
   - Terminal session embedding
   - Demo configuration extraction
   - Quiz and knowledge check parsing

6. **Embedding System**
   - Markdown section imports
   - Transclusion support
   - Dynamic content inclusion
   - Version-aware references

## Data Model

### Graph Structure

```rust
pub struct ContentGraph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

pub enum Node {
    Article { id: String, metadata: ArticleMetadata },
    Concept { name: String, definition: String },
    Resource { url: String, type: ResourceType },
    CodeExample { language: String, content: String },
}

pub enum EdgeType {
    References,
    Requires,
    Extends,
    PartOf,
    RelatedTo,
    ImplementedBy,
}

pub struct Edge {
    from: NodeId,
    to: NodeId,
    edge_type: EdgeType,
    weight: f32,
}
```

### Metadata Extensions

```toml
# Standard metadata
[metadata]
title = "Article Title"
date = "2024-01-01"
author = "Author Name"

# References and resources
[[references]]
title = "Reference Title"
authors = ["Author 1", "Author 2"]
year = 2024
url = "https://example.com"
type = "paper" # paper, book, article, video, documentation

[[resources]]
title = "Helpful Resource"
url = "https://example.com"
description = "What this resource provides"

# Interactive elements
[interactive.terminal]
enabled = true
default_directory = "/project"
startup_commands = ["npm install", "npm run dev"]

[interactive.demo]
enabled = true
url = "https://demo.example.com"
embed_type = "iframe" # iframe, link, sandbox

# Embedded sections
[[embeds]]
source = "other-article.md"
section = "## Installation"
```

## Usage Examples

### Basic Parsing

```rust
use advanced_markdown_parser::{Parser, ParserOptions};

let content = std::fs::read_to_string("article.md")?;
let options = ParserOptions::default();
let parsed = Parser::parse(&content, options)?;

// Access metadata
println!("Title: {}", parsed.metadata.title);
println!("References: {}", parsed.references.len());

// Build knowledge graph
let graph = parsed.build_graph()?;
println!("Nodes: {}, Edges: {}", graph.nodes.len(), graph.edges.len());
```

### Extract Interactive Elements

```rust
let interactive = parsed.extract_interactive_elements()?;

for terminal in interactive.terminals {
    println!("Terminal config: {:?}", terminal);
}

for demo in interactive.demos {
    println!("Demo URL: {}", demo.url);
}
```

### AI Integration

```rust
use advanced_markdown_parser::ai::{generate_summary, extract_key_concepts};

// Generate AI summary
let summary = generate_summary(&parsed, SummaryOptions {
    max_length: 150,
    style: SummaryStyle::Technical,
})?;

// Extract key concepts
let concepts = extract_key_concepts(&parsed)?;
for concept in concepts {
    println!("Concept: {} (confidence: {})", concept.name, concept.confidence);
}
```

## Features Roadmap

See [TODO.md](TODO.md) for detailed implementation plan.

## Integration

This crate is designed to integrate with the main blogger application but can be used standalone or in other projects requiring advanced markdown parsing.

```toml
[dependencies]
advanced_markdown_parser = { path = "../advanced_markdown_parser" }
```

## Contributing

This is an experimental crate exploring the boundaries of what's possible with markdown-based content systems. Ideas and contributions welcome!

## License

Same as parent project
