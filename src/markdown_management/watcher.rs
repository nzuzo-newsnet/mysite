use std::path::Path;
use std::time::Duration;
use notify_debouncer_full::{new_debouncer, notify::*, DebounceEventResult};

/// Start watching the articles directory for changes
/// Changes will be reflected within 5 seconds due to cache TTL
#[cfg(feature = "server")]
pub fn start_article_watcher() -> std::io::Result<()> {
    use std::thread;

    let articles_path = Path::new("articles");

    // Create a debouncer to avoid multiple rapid events
    let mut debouncer = new_debouncer(
        Duration::from_secs(2),
        None,
        move |result: DebounceEventResult| {
            match result {
                Ok(events) => {
                    for event in events {
                        // Check if it's a markdown file
                        let is_markdown = event.paths.iter().any(|path| {
                            path.extension()
                                .and_then(|s| s.to_str())
                                .map(|s| s == "md")
                                .unwrap_or(false)
                        });

                        if is_markdown {
                            dioxus::logger::tracing::info!(
                                "Article file changed: {:?} - Changes will appear within 5 seconds",
                                event.paths
                            );
                        }
                    }
                }
                Err(errors) => {
                    for error in errors {
                        dioxus::logger::tracing::error!("File watcher error: {:?}", error);
                    }
                }
            }
        },
    ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Watch the articles directory recursively
    debouncer
        .watch(articles_path, RecursiveMode::Recursive)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    dioxus::logger::tracing::info!(
        "Started watching articles directory: {:?}",
        articles_path
    );

    // Keep the debouncer alive in a background thread
    // The debouncer must stay in scope for the watcher to work
    thread::spawn(move || {
        let _debouncer = debouncer;
        // Keep the thread alive indefinitely to maintain the file watcher
        loop {
            thread::sleep(Duration::from_secs(60));
        }
    });

    Ok(())
}
