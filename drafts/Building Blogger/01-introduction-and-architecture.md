#####
date = "2025-11-20"
author = "Nzuzo Magagula"
summary = "Building a modern fullstack blog application with Dioxus - learn how to create a type-safe, performant web app that works on both server and client"
thumbnail = "https://i.postimg.cc/pdKhS5Rk/blogger-architecture.png"
category = "Technical"
show_references = true

[[article_series]]
name = "Building Blogger"
next = "Building Blogger/02-markdown-parser-and-rendering"
#####
# Building Blogger: A Fullstack Dioxus Blog Application - Part 1

## Introduction

In this article series, I'll walk you through building a complete blog application using Dioxus - Rust's premier fullstack framework. Unlike typical tutorials that show simplified examples, this series documents the creation of a real, production-ready blogging platform with features like:

- **Fullstack Architecture**: Server-side rendering with client-side hydration
- **Markdown Support**: Custom parser with TOML metadata and series support
- **Browser Caching**: IndexedDB integration using netabase_store
- **File System Watching**: Auto-reload when articles change
- **GitHub Integration**: Display repositories with API caching
- **Responsive Design**: Tailwind CSS with theme support
- **Type Safety**: Fully type-safe from server to client

## Why Dioxus?

Dioxus brings React-like ergonomics to Rust with several key advantages:

### 1. Write Once, Run Anywhere
```rust
#[component]
fn MyComponent(name: String) -> Element {
    rsx! {
        div { "Hello, {name}!" }
    }
}
```

This component works on:
- **Web**: Compiles to WASM
- **Desktop**: Uses webview
- **Mobile**: iOS and Android
- **Server**: SSR for SEO

### 2. Fullstack with Server Functions
```rust
#[server]
async fn fetch_article(path: String) -> Result<Article, ServerError> {
    // This runs on the server only
    let content = tokio::fs::read_to_string(&path).await?;
    Ok(parse_article(content))
}

// Call from client code
let article = fetch_article("post.md".to_string()).await?;
```

The `#[server]` macro generates:
- Server-side implementation
- Client-side RPC stub
- Automatic serialization
- Type-safe communication

### 3. Reactive State Management
```rust
let mut count = use_signal(|| 0);

rsx! {
    button {
        onclick: move |_| count += 1,
        "Clicked {count} times"
    }
}
```

Signal-based reactivity ensures:
- Fine-grained updates
- Minimal re-renders
- Easy to reason about
- No complex lifecycle hooks

## Project Architecture

The blogger application is organized into several key modules:

```
blogger/
├── src/
│   ├── main.rs                     # Entry point and routing
│   ├── markdown_management/        # Article and content management
│   │   ├── local.rs                # File system operations
│   │   ├── github.rs               # GitHub API integration
│   │   ├── github_cache.rs         # IndexedDB caching (WASM)
│   │   └── watcher.rs              # File watching (server)
│   ├── pages/                      # Page components
│   │   ├── home_page/              # Landing page
│   │   ├── article_page/           # Article viewer
│   │   ├── series_page.rs          # Series listing
│   │   └── ...
│   └── shared/                     # Shared components
│       └── nav_bar.rs              # Navigation
├── articles/                       # Markdown content
│   ├── netabase_store/             # Tutorial series
│   ├── Project Management/         # Another series
│   └── ...
└── assets/                         # Static assets
```

### Module Breakdown

#### 1. Main Entry Point (`main.rs`)

```rust
#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/article/:..segments")]
    Article { segments: Vec<String> },
    #[route("/series")]
    Series {},
    #[route("/series/:name")]
    SeriesDetail { name: String },
    // ...
}

fn main() {
    // Start file watcher on server
    #[cfg(feature = "server")]
    {
        if let Err(e) = markdown_management::start_article_watcher() {
            logger::error!("Failed to start watcher: {}", e);
        }
    }

    dioxus::launch(App);
}
```

**Key features:**
- Type-safe routing with `Routable` derive
- Catch-all segments for nested paths
- Conditional compilation for server/client
- Automatic file watching in development

#### 2. Markdown Management (`markdown_management/`)

This module handles all content operations:

```rust
// Server-side: Read files from disk
#[server]
pub async fn fetch_article_with_metadata(
    path: String
) -> Result<ArticleWithMetadata, ServerFnError> {
    let content = tokio::fs::read_to_string(&full_path).await?;
    let toml_metadata = parse_toml_metadata(&content);

    Ok(ArticleWithMetadata {
        metadata: ArticleMetadata {
            name: filename,
            path: relative_path,
            title: extract_title(&content),
        },
        toml_metadata,
        content: extract_content_without_metadata(&content),
    })
}
```

**Metadata format:**
```rust
#[derive(Serialize, Deserialize)]
pub struct ArticleTomlMetadata {
    pub date: Option<String>,
    pub author: Option<String>,
    pub summary: Option<String>,
    pub thumbnail: Option<String>,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub article_series: Vec<ArticleSeries>,
    // ... more fields
}
```

Articles use TOML frontmatter:
```markdown
#####
date = "2025-11-20"
author = "Nzuzo Magagula"
summary = "Article summary"
category = "Technical"

[[article_series]]
name = "Building Blogger"
next = "Building Blogger/02-markdown-parser"
#####
# Article Title

Content here...
```

#### 3. Page Components (`pages/`)

Each page is a Dioxus component:

```rust
#[component]
pub fn ArticlePage(path: String) -> Element {
    // Fetch article data
    let article_data = use_resource(move || async move {
        fetch_article_with_metadata(path.clone()).await
    });

    rsx! {
        main {
            class: "container",
            match article_data.read().as_ref() {
                Some(Ok(article)) => rsx! {
                    Markdown { content: article.content.clone() }
                    NavigationCards { metadata: article.toml_metadata }
                },
                Some(Err(e)) => rsx! {
                    div { "Error: {e}" }
                },
                None => rsx! {
                    div { "Loading..." }
                }
            }
        }
    }
}
```

**Key patterns:**
- `use_resource` for async data fetching
- Pattern matching on `Resource` state
- Automatic re-fetching on dependencies change
- Type-safe prop passing

## Key Design Decisions

### 1. Server Functions Over REST APIs

Instead of defining REST endpoints manually:

```rust
// ❌ Traditional approach
#[get("/api/article/<path>")]
async fn get_article(path: String) -> Json<Article> {
    // Implementation
}

// Client side
let response = fetch("/api/article/post.md").await?;
let article: Article = response.json().await?;
```

We use server functions:

```rust
// ✅ Dioxus approach
#[server]
async fn fetch_article(path: String) -> Result<Article, ServerFnError> {
    // Implementation
}

// Client side (identical to server!)
let article = fetch_article("post.md".to_string()).await?;
```

**Benefits:**
- No URL construction
- No manual serialization
- Type-safe RPC
- Shared code between client and server

### 2. Feature-Gated Code

The same codebase works on different platforms:

```rust
// Always compiled
pub struct ArticleMetadata {
    pub name: String,
    pub path: String,
}

// Server only
#[cfg(feature = "server")]
async fn read_from_filesystem(path: &str) -> Result<String> {
    tokio::fs::read_to_string(path).await
}

// Web only
#[cfg(feature = "web")]
async fn read_from_indexeddb(key: &str) -> Result<String> {
    // IndexedDB operations
}
```

This enables:
- Conditional dependencies (no tokio in WASM)
- Platform-specific optimizations
- Single codebase for all targets

### 3. Signal-Based Reactivity

Instead of useState and useEffect:

```rust
// Create reactive signal
let mut count = use_signal(|| 0);

// Read the value
let current = count();

// Update the value
count += 1;

// Derived computation
let doubled = use_memo(move || count() * 2);

// Effects
use_effect(move || {
    println!("Count changed to: {}", count());
});
```

**Advantages:**
- Automatic dependency tracking
- No dependency arrays
- Fine-grained reactivity
- Copy semantics (cheap clones)

### 4. File System Organization

Articles are organized by series:

```
articles/
├── Building Blogger/
│   ├── 01-introduction.md
│   ├── 02-markdown.md
│   └── summary.md
└── netabase_store/
    ├── 01-introduction.md
    ├── 02-macros.md
    └── summary.md
```

The system:
- Auto-detects series from folder names
- Parses navigation from TOML metadata
- Generates series pages automatically
- Supports multi-level navigation

## Data Flow

### Server-Side Rendering (SSR)

1. **Initial Request**:
   ```
   Browser → Server → Read filesystem → Parse markdown → Render HTML → Browser
   ```

2. **Hydration**:
   ```
   Browser loads WASM → Hydrate components → Attach event handlers → Interactive
   ```

3. **Subsequent Navigation**:
   ```
   Click link → Client-side routing → Call server function → Update UI
   ```

### Client-Side Caching

For GitHub data (WASM only):

```rust
#[cfg(feature = "web")]
pub async fn fetch_github_repos_cached() -> Result<Vec<GitHubRepo>> {
    // Try cache first
    if let Some(cached) = get_from_indexeddb("github_repos").await? {
        if !is_stale(cached.cached_at) {
            return Ok(cached.repos);
        }
    }

    // Fetch fresh data
    let repos = fetch_github_repos().await?;

    // Update cache
    save_to_indexeddb("github_repos", &repos).await?;

    Ok(repos)
}
```

This uses `netabase_store` for type-safe IndexedDB operations:

```rust
#[derive(NetabaseModel, bincode::Encode, bincode::Decode)]
#[netabase(GitHubCacheDefinition)]
pub struct CachedGitHubData {
    #[primary_key]
    pub cache_key: String,
    pub repos: Vec<GitHubRepo>,
    pub cached_at: f64, // Timestamp
}
```

## Performance Considerations

### WASM Bundle Size

The application uses aggressive optimization:

```toml
[profile.release]
opt-level = 'z'     # Optimize for size
lto = true          # Link Time Optimization
codegen-units = 1   # Better optimization
panic = 'abort'     # Reduce binary size
strip = "debuginfo" # Strip debug info

[web.wasm-opt]
level = 'z'         # Maximum compression
```

**Results:**
- Dev build: ~8MB WASM
- Release build: ~500KB WASM (gzipped)
- First Contentful Paint: < 2s

### Lazy Loading

Resources load on-demand:

```rust
// Article only loads when navigated to
let article = use_resource(move || async move {
    fetch_article(path()).await
});

// GitHub repos load in background
let repos = use_resource(|| async move {
    fetch_github_repos_cached().await
});
```

### File Watching (Development)

In development, articles auto-reload:

```rust
#[cfg(feature = "server")]
pub fn start_article_watcher() -> Result<()> {
    let (tx, rx) = channel();

    let mut watcher = notify::recommended_watcher(move |res| {
        tx.send(res).unwrap();
    })?;

    watcher.watch("./articles", RecursiveMode::Recursive)?;

    tokio::spawn(async move {
        while let Ok(event) = rx.recv() {
            // Clear cache
            ARTICLE_CACHE.clear();
        }
    });

    Ok(())
}
```

This enables hot-reloading of content without restarting the server.

## What We'll Build

Over the next four articles, we'll explore:

### Part 2: Markdown Parser and Rendering
- Custom markdown extensions
- TOML frontmatter parsing
- Syntax highlighting
- Math rendering
- Component integration

### Part 3: Article Management and Caching
- File system organization
- Metadata extraction
- Series detection
- IndexedDB caching
- GitHub API integration

### Part 4: Routing and Page Components
- Type-safe routing
- Dynamic segments
- Nested routes
- Navigation components
- Loading states

### Part 5: Fullstack Architecture and Optimization
- Server function patterns
- SSR vs CSR trade-offs
- Bundle size optimization
- Caching strategies
- Production deployment

## Getting Started

To follow along, you'll need:

**Prerequisites:**
- Rust 1.75+ (for async traits)
- Node.js and npm (for Tailwind)
- Dioxus CLI: `cargo install dioxus-cli`

**Create a new project:**
```bash
dx new my-blog --template=fullstack
cd my-blog
dx serve
```

Visit http://localhost:8080 to see your app!

## Conclusion

Dioxus enables building modern web applications in Rust with:
- Fullstack capabilities in one language
- Type safety from server to client
- React-like component model
- Excellent performance

In the next article, we'll dive into the markdown rendering system - how to parse TOML metadata, integrate syntax highlighting, and create a flexible rendering pipeline.

---

**Project Repository**: [github.com/nzuzo-newsnet/blogger](https://github.com/nzuzo-newsnet/blogger)

**Dioxus Documentation**: [dioxuslabs.com](https://dioxuslabs.com)

**Further Reading:**
- [Dioxus Book](https://dioxuslabs.com/learn/0.7/)
- [Server Functions Guide](https://dioxuslabs.com/learn/0.7/reference/server_functions)
- [Fullstack Apps](https://dioxuslabs.com/learn/0.7/cookbook/fullstack)
