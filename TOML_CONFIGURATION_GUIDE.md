# TOML Front Matter Configuration Guide

This guide explains how to configure the TOML front matter in your markdown article files for the blogger platform.

## Table of Contents

1. [Overview](#overview)
2. [Basic Structure](#basic-structure)
3. [Field Reference](#field-reference)
4. [Examples](#examples)
5. [References Configuration](#references-configuration)
6. [Series Navigation](#series-navigation)
7. [Tab Visibility Controls](#tab-visibility-controls)

## Overview

Each markdown article can include TOML metadata at the beginning of the file, enclosed between `#####` delimiters. This metadata controls how the article is displayed, organized, and navigated.

## Basic Structure

The front matter must be placed at the very beginning of your markdown file:

```markdown
#####
date = "2025-11-21"
author = "Your Name"
summary = "A brief description of your article"
#####

# Your Article Title

Your article content starts here...
```

**Important:** Both opening and closing `#####` delimiters are required!

## Field Reference

### Basic Metadata

#### `date` (Optional)
- **Type:** String
- **Format:** `"YYYY-MM-DD"`
- **Description:** Publication date of the article
- **Example:** `date = "2025-11-21"`

#### `author` (Optional)
- **Type:** String
- **Description:** Author's name
- **Example:** `author = "John Doe"`

#### `summary` (Optional)
- **Type:** String
- **Description:** Brief summary/description of the article (displayed in article metadata section)
- **Example:** `summary = "Learn how to build scalable web applications with Rust"`

#### `reading_time` (Optional)
- **Type:** String
- **Description:** Estimated reading time
- **Example:** `reading_time = "10 min read"`

### Categorization

#### `category` (Optional)
- **Type:** String
- **Description:** Primary category for the article
- **Example:** `category = "Tutorial"`
- **Common values:** "Tutorial", "Technical", "Guide", "Opinion", "News"

#### `topics` (Optional)
- **Type:** Array of strings
- **Description:** Main topics covered in the article (displayed as secondary badges)
- **Example:** `topics = ["Rust", "Web Development", "Performance"]`

#### `tags` (Optional)
- **Type:** Array of strings
- **Description:** Tags for searchability (displayed at bottom of article)
- **Example:** `tags = ["async", "tokio", "networking"]`

#### `thumbnail` (Optional)
- **Type:** String (URL)
- **Description:** URL to article thumbnail image
- **Example:** `thumbnail = "https://example.com/image.png"`

### Series Organization

#### `primary_series` (Auto-detected)
- **Type:** String
- **Description:** Automatically detected from folder structure (e.g., articles in `articles/rust-basics/` get `primary_series = "rust-basics"`)
- **Note:** This field is set automatically and should not be manually specified

#### `series` (Optional)
- **Type:** Array of strings
- **Description:** Additional series this article belongs to (for cross-categorization)
- **Example:** `series = ["advanced-topics", "performance-series"]`

#### `article_series` (Optional, Recommended for Series)
- **Type:** Array of tables
- **Description:** Navigation within article series (supports multiple series)
- **Example:**
```toml
[[article_series]]
name = "Building Web Apps"
prev = "part-1-setup"
next = "part-3-deployment"

[[article_series]]
name = "Rust Advanced"
prev = "advanced-traits"
next = "advanced-lifetimes"
```

Each series entry supports:
- `name`: Series name (required)
- `prev`: Path to previous article (optional)
- `next`: Path to next article (optional)

#### Legacy Series Fields (Deprecated)
- `prev_article`: Use `article_series` instead
- `next_article`: Use `article_series` instead

## References Configuration

### Adding References to Your Article

References appear in the "References" tab and provide external resources for readers.

#### Structure

```toml
[[references]]
title = "Reference Title"
url = "https://example.com"
description = "Optional description of what this reference provides"

[[references]]
title = "Another Reference"
url = "https://another-example.com"
```

#### Fields

- `title` (Required): Display name of the reference
- `url` (Required): Full URL to the resource
- `description` (Optional): Brief description of the reference

#### Example

```toml
#####
date = "2025-11-21"
author = "Jane Developer"

[[references]]
title = "The Rust Programming Language Book"
url = "https://doc.rust-lang.org/book/"
description = "Official Rust book covering fundamentals to advanced topics"

[[references]]
title = "Rust API Guidelines"
url = "https://rust-lang.github.io/api-guidelines/"
description = "Best practices for designing Rust APIs"

[[references]]
title = "Tokio Documentation"
url = "https://tokio.rs/tokio/tutorial"
#####

# My Rust Article
```

### Inline Reference Links

While the references section provides external resources, you can still use markdown reference-style links in your content:

```markdown
This uses a [trait][1] for abstraction.

The [associated types][2] make the API cleaner.

[1]: https://doc.rust-lang.org/book/ch10-02-traits.html
[2]: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types
```

The numbered references in the References tab will be separate from these inline links.

## Series Navigation

### Single Series Article

For articles that are part of one series:

```toml
#####
date = "2025-11-21"

[[article_series]]
name = "Rust Basics"
prev = "rust-basics/01-getting-started"
next = "rust-basics/03-ownership"
#####

# Part 2: Variables and Types
```

### Multi-Series Article

An article can belong to multiple series:

```toml
#####
date = "2025-11-21"

[[article_series]]
name = "Web Development"
prev = "web-dev/04-frontend"
next = "web-dev/06-testing"

[[article_series]]
name = "Rust Advanced"
prev = "advanced/02-async"
next = "advanced/04-macros"
#####

# Building Full-Stack Applications
```

## Tab Visibility Controls

Control which tabs are shown in the article sidebar:

### `show_references` (Default: true)
- **Type:** Boolean
- **Description:** Show/hide the References tab
- **Example:** `show_references = false`
- **Note:** References tab is shown by default even if this field is omitted

### `show_demo` (Default: false)
- **Type:** Boolean
- **Description:** Show/hide the Demo tab
- **Example:** `show_demo = true`

### `show_related` (Default: false)
- **Type:** Boolean
- **Description:** Show/hide the Related Articles tab
- **Example:** `show_related = true`

### `show_quiz` (Default: false)
- **Type:** Boolean
- **Description:** Show/hide the Quiz tab
- **Example:** `show_quiz = true`

## Examples

### Minimal Article

```markdown
#####
date = "2025-11-21"
author = "John Doe"
#####

# Simple Article Title

Article content here...
```

### Tutorial Article with References

```markdown
#####
date = "2025-11-21"
author = "Jane Developer"
summary = "Learn how to build REST APIs with Rust and Actix"
category = "Tutorial"
topics = ["Rust", "Web Development", "REST API"]
tags = ["actix", "web", "api", "backend"]
reading_time = "15 min read"
thumbnail = "https://example.com/actix-tutorial.png"

[[references]]
title = "Actix Web Documentation"
url = "https://actix.rs/docs/"
description = "Official Actix-web framework documentation"

[[references]]
title = "REST API Best Practices"
url = "https://restfulapi.net/"
description = "Guide to RESTful API design principles"

[[references]]
title = "Rust Async Book"
url = "https://rust-lang.github.io/async-book/"
#####

# Building REST APIs with Actix-Web

In this tutorial, we'll build a production-ready REST API...
```

### Series Article

```markdown
#####
date = "2025-11-20"
author = "Tech Writer"
summary = "Part 3 of our comprehensive Rust series"
category = "Technical"
topics = ["Rust", "Advanced"]
tags = ["traits", "generics", "type-system"]
reading_time = "20 min read"

[[article_series]]
name = "Mastering Rust"
prev = "rust-series/02-ownership"
next = "rust-series/04-lifetimes"

[[references]]
title = "Rust Book - Traits"
url = "https://doc.rust-lang.org/book/ch10-02-traits.html"

[[references]]
title = "Rust by Example - Generics"
url = "https://doc.rust-lang.org/rust-by-example/generics.html"

show_references = true
show_quiz = true
#####

# Part 3: Advanced Traits and Generics

Building on our previous discussion of ownership...
```

### Article with Multiple Series

```markdown
#####
date = "2025-11-22"
author = "Expert Dev"
summary = "Cross-cutting concepts in web development and Rust"
category = "Technical"
topics = ["Rust", "Web", "Architecture"]

[[article_series]]
name = "Web Development Mastery"
prev = "web/05-state-management"
next = "web/07-deployment"

[[article_series]]
name = "Rust Design Patterns"
prev = "patterns/03-builder"
next = "patterns/05-strategy"

[[references]]
title = "Rust Design Patterns"
url = "https://rust-unofficial.github.io/patterns/"
description = "Comprehensive catalog of Rust design patterns"

show_references = true
show_related = true
#####

# Architectural Patterns for Web Applications
```

## Common Mistakes to Avoid

1. **Missing closing delimiter:** Always include both opening and closing `#####`
   ```markdown
   ❌ Wrong:
   #####
   date = "2025-11-21"
   
   # Article
   
   ✅ Correct:
   #####
   date = "2025-11-21"
   #####
   
   # Article
   ```

2. **Invalid TOML syntax:** Strings must be quoted
   ```toml
   ❌ Wrong: author = John Doe
   ✅ Correct: author = "John Doe"
   ```

3. **Array syntax:** Use square brackets for arrays
   ```toml
   ❌ Wrong: topics = "Rust", "Web"
   ✅ Correct: topics = ["Rust", "Web"]
   ```

4. **Reference tables:** Use `[[references]]` for each entry
   ```toml
   ❌ Wrong:
   references = [
     { title = "Ref 1", url = "http://..." }
   ]
   
   ✅ Correct:
   [[references]]
   title = "Ref 1"
   url = "http://..."
   ```

## Testing Your Configuration

After adding front matter to your article:

1. Save the file
2. Reload your article in the browser
3. Check that:
   - Metadata appears correctly in the article header
   - Tags/topics display properly
   - References tab shows all your references
   - Series navigation works (if applicable)
   - Only enabled tabs are visible in the sidebar

## Questions or Issues?

If you encounter problems with front matter parsing:
- Verify TOML syntax is valid
- Ensure both `#####` delimiters are present
- Check that strings are properly quoted
- Confirm array syntax for multi-value fields

For more help, refer to the TOML specification: https://toml.io/
