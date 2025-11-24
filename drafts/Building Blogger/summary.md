#####
short_summary = "A comprehensive tutorial series on building a production-ready fullstack blog application with Dioxus. Learn how to create a type-safe web app with server-side rendering, client-side hydration, markdown parsing with TOML metadata, IndexedDB caching, GitHub integration, and aggressive WASM optimization - all while maintaining a single Rust codebase that works on web, desktop, and mobile."
name = "summary"
#####
# Building Blogger: A Complete Fullstack Dioxus Application

## Introduction

This series documents the creation of a real, production-ready blog application using Dioxus - Rust's premier fullstack framework. Unlike typical tutorials, we're building an actual blogging platform with all the features you'd expect in a modern web application.

**What makes this different?**
- Real production code, not simplified examples
- Fullstack architecture explained from both server and client perspectives
- Performance optimization with concrete benchmarks
- Type-safe patterns throughout the stack
- Cross-platform considerations (web, desktop, mobile)

## What You'll Build

By following this series, you'll create a complete blog application with:

### Core Features
- **Multi-page Application**: Home, article viewer, series listing, about page
- **Markdown Rendering**: Custom parser with TOML frontmatter and syntax highlighting
- **Article Organization**: Auto-detect series from folder structure
- **Navigation System**: Previous/next links, breadcrumbs, table of contents
- **GitHub Integration**: Display repositories with client-side caching
- **Responsive Design**: Mobile-first with Tailwind CSS and theme support

### Technical Features
- **Server-Side Rendering**: Fast initial page loads for SEO
- **Client-Side Hydration**: Interactive UI after WASM loads
- **Hot Reloading**: Auto-refresh when articles change (development)
- **Type-Safe RPC**: Server functions with automatic serialization
- **Browser Caching**: IndexedDB integration using netabase_store
- **Optimized WASM**: < 500KB production bundle

## Prerequisites

To get the most from this series, you should understand:

**Essential:**
- Rust fundamentals (ownership, traits, async/await)
- Basic web concepts (HTML, CSS, HTTP)
- Component-based UI frameworks (React, Vue, etc.)

**Helpful but not required:**
- Dioxus basics (we explain as we go)
- Fullstack architecture patterns
- Performance optimization techniques

## Series Overview

### Part 1: Introduction and Architecture Overview

**What you'll learn:**
- Why Dioxus for fullstack development
- Project structure and organization
- Routing with type-safe paths
- Server functions vs REST APIs
- Signal-based reactivity
- Feature-gated compilation

**Key concepts:**
```rust
// Server function that runs on server
#[server]
async fn fetch_article(path: String) -> Result<Article> {
    tokio::fs::read_to_string(&path).await
}

// Called from client - looks like normal Rust
let article = fetch_article("post.md".to_string()).await?;
```

**Topics covered:**
- Dioxus architecture and benefits
- fullstack vs traditional approach
- Module organization patterns
- Conditional compilation strategies
- Basic data flow patterns

### Part 2: Markdown Parser and Rendering

**What you'll learn:**
- Parsing TOML frontmatter from markdown
- Integrating `dioxus_markdown` for rendering
- Adding syntax highlighting with Prism.js
- Creating custom markdown extensions
- Component-based rendering pipeline

**Key implementation:**
```rust
/// Parse TOML metadata between ##### delimiters
fn parse_toml_metadata(content: &str) -> Option<ArticleTomlMetadata> {
    const DELIMITER: &str = "#####";

    let first_pos = content.find(DELIMITER)?;
    let after_first = &content[first_pos + DELIMITER.len()..];
    let second_pos = after_first.find(DELIMITER)?;
    let toml_content = &after_first[..second_pos].trim();

    toml::from_str(toml_content).ok()
}
```

**Topics covered:**
- TOML frontmatter structure and parsing
- Markdown AST manipulation
- Syntax highlighting integration
- Math rendering with KaTeX
- Custom block types (callouts, code tabs)
- SEO metadata generation

**Metadata example:**
```toml
#####
date = "2025-11-20"
author = "Nzuzo Magagula"
summary = "Article summary for SEO"
thumbnail = "https://..."
category = "Technical"
tags = ["rust", "dioxus", "tutorial"]

[[article_series]]
name = "Building Blogger"
previous = "Building Blogger/01-introduction"
next = "Building Blogger/03-article-management"
#####
```

### Part 3: Article Management and Caching System

**What you'll learn:**
- File system organization for articles
- Auto-detecting series from folder structure
- Extracting titles without full parsing
- Server-side caching with `cached` crate
- Client-side caching with IndexedDB
- File watching for hot reloading

**File organization:**
```
articles/
├── Building Blogger/
│   ├── 01-introduction.md
│   ├── 02-markdown.md
│   ├── summary.md           # Special file
│   └── assets/
└── netabase_store/
    ├── 01-introduction.md
    ├── 02-macros.md
    └── ...
```

**IndexedDB caching:**
```rust
#[derive(NetabaseModel, bincode::Encode, bincode::Decode)]
#[netabase(GitHubCacheDefinition)]
pub struct CachedGitHubData {
    #[primary_key]
    pub cache_key: String,
    pub repos: Vec<GitHubRepo>,
    pub cached_at: f64, // JavaScript timestamp
}

// Usage
async fn fetch_repos_cached() -> Result<Vec<GitHubRepo>> {
    // Check cache
    if let Some(cached) = get_cached().await? {
        if !is_stale(cached.cached_at) {
            return Ok(cached.repos);
        }
    }

    // Fetch fresh
    let repos = fetch_from_github().await?;
    save_to_cache(&repos).await?;

    Ok(repos)
}
```

**Topics covered:**
- Efficient file system traversal
- Metadata extraction strategies
- Server-side caching patterns
- IndexedDB with netabase_store
- File watching with `notify` crate
- Cache invalidation strategies
- GitHub API integration and rate limiting

### Part 4: Routing and Page Components

**What you'll learn:**
- Type-safe routing with Routable derive
- Dynamic route segments and catch-alls
- Component props and state management
- Loading states and error handling
- Navigation components
- Responsive layouts

**Type-safe routing:**
```rust
#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},

    // Catch-all for nested paths
    #[route("/article/:..segments")]
    Article { segments: Vec<String> },

    #[route("/series")]
    Series {},

    #[route("/series/:name")]
    SeriesDetail { name: String },
}
```

**Component patterns:**
```rust
#[component]
pub fn ArticlePage(path: String) -> Element {
    // Reactive resource - refetches when path changes
    let article_data = use_resource(move || async move {
        fetch_article_with_metadata(path.clone()).await
    });

    rsx! {
        div {
            match article_data.read().as_ref() {
                Some(Ok(article)) => rsx! {
                    ArticleMetadata { meta: article.toml_metadata }
                    Markdown { content: article.content }
                    NavigationCards { meta: article.toml_metadata }
                },
                Some(Err(e)) => rsx! {
                    ErrorDisplay { error: e.to_string() }
                },
                None => rsx! {
                    LoadingSkeleton {}
                }
            }
        }
    }
}
```

**Topics covered:**
- Route definition and navigation
- Props passing and validation
- State management patterns
- Resource loading and caching
- Error boundaries
- Loading skeletons
- Accessibility considerations
- Mobile-first responsive design

### Part 5: Fullstack Architecture and Optimization

**What you'll learn:**
- Server functions deep dive
- SSR vs CSR trade-offs
- WASM bundle optimization
- Code splitting strategies
- Production deployment
- Performance monitoring

**WASM optimization:**
```toml
[profile.release]
opt-level = 'z'         # Optimize for size
lto = true              # Link Time Optimization
codegen-units = 1       # Better optimization
panic = 'abort'         # Reduce binary size
strip = "debuginfo"     # Strip debug info

[web.wasm-opt]
level = 'z'             # Maximum compression

[web.pre-compress]
enabled = true          # Brotli/gzip
```

**Performance results:**
| Metric | Dev Mode | Release Mode |
|--------|----------|--------------|
| WASM Size | ~8MB | ~500KB |
| First Paint | ~53s | ~2s |
| Interaction | ~55s | ~2.5s |
| Bundle Load | ~50s | ~1s |

**Server function patterns:**
```rust
// Simple data fetching
#[server]
async fn fetch_article(path: String) -> Result<String> {
    tokio::fs::read_to_string(&path).await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

// With caching
#[server]
#[cached(time = 300, result = true)]  // Cache for 5 minutes
async fn fetch_article_cached(path: String) -> Result<String> {
    fetch_article(path).await
}

// With authentication
#[server]
async fn update_article(
    path: String,
    content: String,
    auth: String
) -> Result<()> {
    verify_auth(&auth)?;
    tokio::fs::write(&path, content).await?;
    Ok(())
}
```

**Topics covered:**
- Server function best practices
- Caching strategies (server and client)
- Bundle size analysis and reduction
- Code splitting with dynamic imports
- Progressive Web App features
- Docker deployment
- CI/CD pipelines
- Monitoring and analytics
- SEO optimization
- Performance budgets

## Key Techniques and Patterns

Throughout the series, you'll encounter:

### Dioxus Patterns
- Component composition and reusability
- Signal-based reactivity and effects
- Resource management and lifecycle
- Context for global state
- Conditional rendering strategies

### Fullstack Patterns
- Server functions for RPC
- Type-safe client-server communication
- Feature-gated code for different platforms
- SSR with client hydration
- Progressive enhancement

### Performance Patterns
- Lazy loading and code splitting
- Aggressive WASM optimization
- Multi-level caching strategies
- Efficient file system operations
- Minimal re-renders

### Architecture Patterns
- Module organization for scalability
- Separation of concerns
- Dependency injection
- Error handling strategies
- Testing approaches

## What Makes This Series Unique

### 1. Real Production Code
Every example comes from the actual blogger application. You're learning from real-world code, not simplified demos.

### 2. Fullstack Perspective
We cover both server and client sides, showing how they integrate and where responsibilities split.

### 3. Performance Focus
Concrete benchmarks and optimization strategies, not just "make it work" solutions.

### 4. Type Safety Throughout
Leverage Rust's type system for correctness from server to client.

### 5. Cross-Platform Considerations
Write once, deploy to web, desktop, and mobile.

## Project Structure

The final application has this structure:

```
blogger/
├── src/
│   ├── main.rs                 # Entry point, routing
│   ├── markdown_management/    # Content management
│   │   ├── mod.rs
│   │   ├── local.rs            # File system ops
│   │   ├── github.rs           # GitHub API
│   │   ├── github_cache.rs     # IndexedDB cache (web)
│   │   └── watcher.rs          # File watching (server)
│   ├── pages/                  # Page components
│   │   ├── home_page/
│   │   │   ├── mod.rs
│   │   │   ├── blog.rs         # Blog section
│   │   │   └── projects.rs     # Projects section
│   │   ├── article_page/
│   │   │   └── mod.rs
│   │   ├── series_page.rs
│   │   ├── series_detail_page.rs
│   │   ├── about_page.rs
│   │   ├── demos_page.rs
│   │   └── reading_page.rs
│   └── shared/                 # Shared components
│       ├── mod.rs
│       └── nav_bar.rs
├── articles/                   # Markdown content
│   ├── Building Blogger/
│   ├── netabase_store/
│   └── Project Management/
├── assets/                     # Static files
│   └── tailwind.css
├── Cargo.toml                  # Dependencies
├── Dioxus.toml                 # Dioxus configuration
└── PERFORMANCE.md              # Performance notes
```

## Dependencies

Key dependencies used:

```toml
[dependencies]
dioxus = { version = "0.7.0-rc-3", features = ["router", "fullstack"] }
dioxus_markdown = "..." # Markdown rendering
tokio = "1.48.0"       # Async runtime (server)
serde = "1.0"          # Serialization
anyhow = "1.0"         # Error handling
cached = "0.54"        # Server-side caching
notify = "7.0"         # File watching
netabase_store = "..." # IndexedDB (web)
```

## Development Workflow

Typical development session:

```bash
# Start dev server
dx serve

# In another terminal, watch for changes
dx watch

# Build for production
dx build --release

# Deploy
docker build -t blogger .
docker run -p 8080:8080 blogger
```

## Common Patterns

### Fetching Data
```rust
// Define server function
#[server]
async fn fetch_data() -> Result<Data> {
    // Server-side logic
}

// Use in component
let data = use_resource(|| async move {
    fetch_data().await
});

// Render
match data.read().as_ref() {
    Some(Ok(d)) => rsx! { /* render data */ },
    Some(Err(e)) => rsx! { /* error */ },
    None => rsx! { /* loading */ },
}
```

### Navigation
```rust
let nav = navigator();

// Navigate programmatically
nav.push(Route::Article {
    segments: vec!["post".to_string()]
});

// Or use Link component
rsx! {
    Link {
        to: Route::Article { segments: vec!["post".to_string()] },
        "Read Article"
    }
}
```

### State Management
```rust
// Local state
let mut count = use_signal(|| 0);

// Global state with context
let ctx: MyContext = use_context();

// Derived state
let doubled = use_memo(move || count() * 2);

// Effects
use_effect(move || {
    println!("Count: {}", count());
});
```

## Expected Outcomes

After completing this series, you'll be able to:

- Build fullstack web applications in Rust
- Understand Dioxus architecture and patterns
- Implement server-side rendering
- Optimize WASM bundle size
- Create type-safe APIs
- Integrate third-party services
- Deploy to production
- Debug and profile performance

## Beyond the Basics

The techniques learned here apply to:

- **Admin Dashboards**: Build internal tools
- **E-commerce**: Product catalogs with SSR
- **Documentation Sites**: Like this blog but for docs
- **Real-time Apps**: Add WebSocket support
- **Mobile Apps**: Same code, different platform

## Start Your Journey

Ready to build modern web apps in Rust?

**[Part 1: Introduction and Architecture Overview →](./01-introduction-and-architecture.md)**

---

## About the Project

This blog application is used in production at NewsNet Africa to publish technical articles, tutorials, and project documentation. It demonstrates that Rust and Dioxus are ready for real-world web development.

**Live Demo**: [blog.nzuzo.dev](https://blog.nzuzo.dev) _(example)_

**Source Code**: [github.com/nzuzo-newsnet/blogger](https://github.com/nzuzo-newsnet/blogger)

**Dioxus**: [dioxuslabs.com](https://dioxuslabs.com)

## License

Articles: CC BY-SA 4.0
Code: MIT OR Apache-2.0
