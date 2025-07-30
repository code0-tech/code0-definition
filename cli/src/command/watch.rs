use crate::table::error_table;
use code0_definition_reader::parser::Parser;
use colored::Colorize;
use notify::{Event, EventKind, RecursiveMode, Watcher, recommended_watcher};
use std::sync::mpsc::channel;

pub async fn watch_for_changes(path: Option<String>) {
    let dir_path = path.unwrap_or_else(|| "./definitions".to_string());

    println!(
        "{}",
        format!("Watching directory: {dir_path}")
            .bright_yellow()
            .bold()
    );
    println!("{}", "Press Ctrl+C to stop watching...".dimmed());

    {
        let parser = match Parser::from_path(dir_path.as_str()) {
            Some(reader) => reader,
            None => {
                panic!("Error reading definitions");
            }
        };

        error_table(&parser.features);
    }

    // Set up file watcher
    let (tx, rx) = channel();
    let mut watcher = recommended_watcher(tx).unwrap();
    watcher
        .watch(std::path::Path::new(&dir_path), RecursiveMode::Recursive)
        .unwrap();

    loop {
        match rx.recv() {
            Ok(event) => match event {
                Ok(Event {
                    kind: EventKind::Create(_),
                    ..
                })
                | Ok(Event {
                    kind: EventKind::Modify(_),
                    ..
                })
                | Ok(Event {
                    kind: EventKind::Remove(_),
                    ..
                }) => {
                    println!(
                        "\n{}",
                        "Change detected! Regenerating report...".bright_yellow()
                    );

                    let parser = match Parser::from_path(dir_path.as_str()) {
                        Some(reader) => reader,
                        None => {
                            panic!("Error reading definitions");
                        }
                    };

                    error_table(&parser.features);
                }
                _ => {}
            },
            Err(e) => println!("Watch error: {e:?}"),
        }
    }
}
