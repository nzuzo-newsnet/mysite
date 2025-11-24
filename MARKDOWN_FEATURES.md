# Markdown Features Documentation

This document describes the markdown parsing system used in the blogger application, including frontmatter configuration, article organization, and navigation features.

## Table of Contents

- [TOML Frontmatter](#toml-frontmatter)
- [Article Navigation](#article-navigation)
- [Series Organization](#series-organization)
- [Reference Lists](#reference-lists)
- [Custom Features](#custom-features)
- [Examples](#examples)

## TOML Frontmatter

Articles use TOML frontmatter enclosed between `#####` delimiters for metadata configuration. The frontmatter must appear at the beginning of the file.

### Basic Structure

```markdown
#####
date = "2025-11-20"
author = "John Doe"
summary = "A brief description of the article"
#####
# Article Title

Article content goes here...
```

### Available Fields

#### Required Fields

None of the fields are strictly required, but the following are recommended for proper article display:

- `date` (String): Publication date in YYYY-MM-DD format
- `author` (String): Author name
- `summary` (String): Brief description displayed in article lists

#### Optional Metadata Fields

- `topics` (Array of Strings): Main topics covered
  ```toml
  topics = ["Rust", "Databases", "Backend Development"]
  ```

- `tags` (Array of Strings): Searchable tags for categorization
  ```toml
  tags = ["tutorial", "advanced", "architecture"]
  ```

- `thumbnail` (String): URL to article thumbnail image
  ```toml
  thumbnail = "https://example.com/image.png"
  ```

- `reading_time` (String): Estimated reading time
  ```toml
  reading_time = "10 min read"
  ```

- `category` (String): Article category
  ```toml
  category = "Technical"
  ```

## Article Navigation

### Previous and Next Articles

You can link articles together in a sequence using the `article_series` array. This is the recommended modern approach for article navigation.

#### Using article_series (Recommended)

```toml
#####
[[article_series]]
name = "Building netabase_store"
prev = "netabase_store/01-introduction-and-overview"
next = "netabase_store/03-backend-implementation-and-trait-design"
#####
```

Multiple series navigation can be defined for articles that belong to multiple series:

```toml
#####
[[article_series]]
name = "Rust Fundamentals"
prev = "rust-basics/01-ownership"
next = "rust-basics/03-borrowing"

[[article_series]]
name = "Systems Programming"
prev = "systems/memory-management"
next = "systems/concurrency"
#####
```

#### Legacy Navigation (Deprecated)

The older `prev_article` and `next_article` fields are still supported but should be avoided in new articles:

```toml
#####
prev_article = "previous-article-name"
next_article = "next-article-name"
#####
```

**Note:** Use `article_series` instead for better organization and multi-series support.

## Series Organization

Series allow you to group related articles together. The system supports both automatic series detection and manual configuration.

### Automatic Series Detection

Articles placed in subdirectories under `articles/` are automatically assigned to a series based on the folder structure:

```
articles/
├── netabase_store/
│   ├── 01-introduction-and-overview.md
│   ├── 02-procedural-macros.md
│   └── 03-backend-implementation.md
└── Project Management/
    ├── 01-planning-and-scope.md
    └── 02-people.md
```

The `primary_series` field is automatically set based on the folder path:
- `articles/netabase_store/01-intro.md` → `primary_series = "netabase_store"`
- `articles/Project Management/01-planning.md` → `primary_series = "Project Management"`

### Manual Series Assignment

You can manually assign articles to additional series using the `series` array:

```toml
#####
series = ["Advanced Topics", "Best Practices"]
#####
```

This allows an article to appear in multiple series simultaneously.

### Series Summary Files

Each series directory can contain a `summary.md` file that provides:
- A short summary for series listings
- A longer markdown description displayed on the series page

Example `summary.md` structure:

```markdown
#####
short_summary = "A comprehensive guide to building type-safe database abstractions in Rust"
#####
# Building netabase_store Series

This series walks through the design and implementation of netabase_store,
a type-safe, multi-backend database abstraction library...

## What You'll Learn
- Procedural macro design
- Trait-based abstractions
- Performance optimization techniques
```

### Series Navigation Format

When using `article_series`, you can specify:

- `name`: The series identifier (must match the series name)
- `prev`: Path to the previous article (relative to `articles/` directory, without `.md` extension)
- `next`: Path to the next article (relative to `articles/` directory, without `.md` extension)

```toml
[[article_series]]
name = "My Series"
prev = "folder/previous-article"    # Links to articles/folder/previous-article.md
next = "folder/next-article"        # Links to articles/folder/next-article.md
```

## Reference Lists

Reference lists can be toggled using the `show_references` field. When enabled, references appear at the bottom of the article.

```toml
#####
show_references = true  # Default: true
#####
```

To hide references:

```toml
#####
show_references = false
#####
```

References should be defined in your markdown using standard markdown reference syntax:

```markdown
This is a sentence with a reference [1].

## References

1. Smith, J. (2024). "Title of the Paper"
2. Doe, J. (2024). "Another Reference"
```

## Custom Features

### Bottom Navigation Controls

Control which bottom navigation sections appear:

```toml
#####
show_references = true   # Show references section (default: true)
show_demo = false        # Show demo section (default: false)
show_related = false     # Show related articles (default: false)
show_quiz = false        # Show quiz section (default: false)
#####
```

### Nested Series (Advanced)

The `SeriesInfo` structure supports nested series up to 2 levels deep through the `parent` field:

```rust
// This is auto-generated from folder structure
SeriesInfo {
    name: "Advanced Database Topics",
    part: 3,
    total_parts: Some(5),
    parent: Some(Box::new(SeriesInfo {
        name: "Database Fundamentals",
        part: 2,
        total_parts: Some(3),
        parent: None,
    }))
}
```

Note: Manual nested series configuration in TOML is not currently supported. Use folder structure for automatic nested series detection.

## Examples

### Complete Article Example

```markdown
#####
date = "2025-11-20"
author = "Nzuzo Magagula"
summary = "An introduction to building a type-safe, multi-backend database abstraction library"
thumbnail = "https://example.com/thumbnail.png"
category = "Technical"
topics = ["Rust", "Databases", "Architecture"]
tags = ["tutorial", "advanced", "type-safety"]
reading_time = "15 min read"
show_references = true
show_demo = false

[[article_series]]
name = "Building netabase_store"
next = "netabase_store/02-procedural-macros-and-code-generation"
#####
# Building netabase_store: A Type-Safe Multi-Backend Database Abstraction — Part 1

## Introduction

This article introduces...

## Main Content

...

## References

1. Rust Programming Language Documentation
2. serde Documentation
```

### Multi-Series Article Example

```markdown
#####
date = "2025-11-15"
author = "Jane Developer"
summary = "Advanced patterns for concurrent programming in Rust"
topics = ["Rust", "Concurrency"]
tags = ["advanced", "multithreading", "async"]

# This article belongs to multiple series
series = ["Rust Advanced Topics", "Systems Programming"]

[[article_series]]
name = "Rust Advanced Topics"
prev = "advanced/ownership-patterns"
next = "advanced/unsafe-rust"

[[article_series]]
name = "Systems Programming"
prev = "systems/memory-management"
next = "systems/performance-optimization"
#####
# Concurrent Programming in Rust

...content...
```

### Minimal Article Example

```markdown
#####
date = "2025-11-01"
author = "Quick Writer"
summary = "A quick guide to getting started"
#####
# Getting Started

This is a simple article with minimal metadata...
```

## File Organization Best Practices

1. **Use descriptive folder names** for series:
   ```
   articles/
   ├── rust-fundamentals/
   ├── web-development/
   └── database-design/
   ```

2. **Number articles** in a series for automatic sorting:
   ```
   01-introduction.md
   02-basics.md
   03-advanced.md
   ```

3. **Include summary.md** for each series to provide context

4. **Use consistent naming** for article paths in navigation:
   ```toml
   # Bad - inconsistent casing
   prev = "myFolder/MyArticle"

   # Good - consistent lowercase with hyphens
   prev = "my-folder/my-article"
   ```

## Markdown Rendering

The system uses a custom markdown parser that supports:
- Standard markdown syntax (headings, lists, links, code blocks, etc.)
- Syntax highlighting for code blocks
- Image embedding
- Tables
- Blockquotes

Code blocks support syntax highlighting:

````markdown
```rust
fn main() {
    println!("Hello, world!");
}
```
````

## Caching

All article fetching is cached with a 5-second TTL to improve performance:
- `list_files()`: Lists all available articles
- `fetch_article_content()`: Fetches raw article content
- `fetch_article_with_metadata()`: Fetches article with parsed metadata
- `fetch_all_series()`: Fetches all series with their articles
- `fetch_series_by_name()`: Fetches a specific series

## Path Structure

Articles are stored in the `articles/` directory at the project root:

```
blogger/
├── articles/           # All article content
│   ├── series-name/    # Series subdirectories
│   │   ├── 01-article.md
│   │   ├── 02-article.md
│   │   └── summary.md
│   └── standalone-article.md
└── aboutme.md         # About page content
```

When referencing articles in navigation, use paths relative to the `articles/` directory without the `.md` extension:
- File: `articles/netabase_store/01-intro.md`
- Reference: `netabase_store/01-intro`

## Troubleshooting

### Article Not Appearing

- Check that the file has a `.md` extension
- Ensure the file is in the `articles/` directory or a subdirectory
- Verify the TOML frontmatter is properly formatted between `#####` delimiters

### Navigation Not Working

- Ensure article paths in `prev`/`next` don't include the `.md` extension
- Verify paths are relative to the `articles/` directory
- Check that the series `name` matches exactly in all articles of the series

### Series Not Displaying

- Make sure articles in the series have the correct `primary_series` or `series` field
- Verify the folder structure matches the series name
- Check that `summary.md` exists if you want a series description
- Ensure `summary.md` files are not appearing as articles (they are automatically filtered out)

## Technical Notes

### Metadata Parsing

The system uses the `toml` crate to parse frontmatter. The delimiters (`#####`) mark the boundaries:

```
[start of file]
#####              ← First delimiter
[TOML content]
#####              ← Second delimiter
[Markdown content]
```

### Content Extraction

When articles are fetched with metadata:
1. TOML frontmatter is parsed and validated
2. Content between delimiters is removed
3. Title is extracted from the first `# ` heading
4. Primary series is auto-detected from folder structure

### Performance Considerations

- Article listing only reads the first 10 lines to extract titles
- Full content is only loaded when specifically requested
- All operations are cached to reduce filesystem access
- Parallel fetching is used when loading multiple articles
