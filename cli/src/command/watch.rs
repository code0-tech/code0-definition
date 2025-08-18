use crate::analyser::Analyser;
use crate::formatter::{default, info};
use notify::event::ModifyKind;
use notify::{EventKind, RecursiveMode, Watcher, recommended_watcher};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

pub async fn watch_for_changes(path: Option<String>) {
    let dir_path = path.unwrap_or_else(|| "./definitions".to_string());

    info(format!("Watching directory: {dir_path}"));
    info(String::from("Press Ctrl+C to stop watching..."));

    {
        Analyser::new(dir_path.as_str()).report(false);
    }

    // Set up file watcher
    let (tx, rx) = channel();
    let mut watcher = recommended_watcher(tx).unwrap();
    watcher
        .watch(std::path::Path::new(&dir_path), RecursiveMode::Recursive)
        .unwrap();

    let mut last_run = Instant::now();

    loop {
        if let Ok(Ok(event)) = rx.recv() {
            match event.kind {
                EventKind::Modify(modify) => {
                    if let ModifyKind::Data(_) = modify
                        && last_run.elapsed() > Duration::from_millis(500)
                    {
                        default(String::from(
                            "\n\n\n--------------------------------------------------------------------------\n\n",
                        ));
                        info(String::from("Change detected! Regenerating report..."));
                        Analyser::new(dir_path.as_str()).report(false);
                        last_run = Instant::now();
                    }
                }
                EventKind::Remove(_) => {
                    if last_run.elapsed() > Duration::from_millis(500) {
                        default(String::from(
                            "\n\n\n--------------------------------------------------------------------------\n\n",
                        ));
                        info(String::from("Change detected! Regenerating report..."));
                        Analyser::new(dir_path.as_str()).report(false);
                        last_run = Instant::now();
                    }
                }
                _ => {}
            }
        }
    }
}
