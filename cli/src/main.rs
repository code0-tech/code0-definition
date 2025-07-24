use clap::{Parser as ClapParser, Subcommand};
use reader::parser::Parser;
use reader::parser::DefinitionError;
use reader::parser::Feature;
use tucana::shared::{RuntimeFunctionDefinition, DefinitionDataType, FlowType};
use notify::{Watcher, RecursiveMode, Event, EventKind, recommended_watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use colored::*;
use crate::table::*;

mod table;

/// Top-level CLI for 'definition'
#[derive(ClapParser)]
#[command(name = "definition")]
#[command(version = "1.0")]
#[command(about = "Manage definitions, reports, and features")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a general report.
    Report {
        /// Optional path to root directory of all definitions.
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Generate a report for a or all feature(s).
    Feature {
        /// Optional name of the definition set.
        #[arg(short, long)]
        featurename: Option<String>,
        /// Optional path to root directory of all definitions.
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Look up a specific definition.
    Definition {
        /// Required name of the definition.
        #[arg(short, long)]
        name: String,
        /// Optional path to root directory of all definitions.
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Watch for changes to and regenerate error reports.
    Watch {
        /// Optional path to root directory of all definitions.
        #[arg(short, long)]
        path: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Report { path } => {
            let dir_path = match path {
                Some(p) => p,
                None => "../definitions".to_string(),
            };

            let parser = match Parser::from_path(dir_path.as_str()) {
                Some(reader) => reader,
                None => {
                    panic!("Error reading definitions");
                }
            };
            error_table(&parser.features);
            summary_table(&parser.features);
        }
        Commands::Feature { featurename, path } => {
            let dir_path = match path {
                Some(p) => p,
                None => "../definitions".to_string(),
            };

           let parser = match Parser::from_path(dir_path.as_str()) {
               Some(reader) => reader,
               None => {
                   panic!("Error reading definitions");
               }
           };

           if let Some(featurename) = featurename {
               let mut features_to_report = Vec::new();
               for feature in &parser.features {
                   if feature.name == featurename {
                       feature_table(&feature);
                       features_to_report.push(feature.clone());
                   }
               }
               summary_table(&features_to_report);
           } else {
               for feature in &parser.features {
                   feature_table(&feature);
               }
               summary_table(&parser.features);
           }
        }
        Commands::Definition { name, path } => {
            println!("Handling definition with name: {}", name);
            let dir_path = match path {
                Some(p) => p,
                None => "../definitions".to_string(),
            };

            todo!("Implement definition query and display command!");
        }
        Commands::Watch { path } => {
            let dir_path = match path {
                Some(p) => p,
                None => "../definitions".to_string(),
            };

            println!("{}", format!("Watching directory: {}", dir_path).bright_yellow().bold());
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
            watcher.watch(std::path::Path::new(&dir_path), RecursiveMode::Recursive).unwrap();

            loop {
                match rx.recv() {
                    Ok(event) => {
                        match event {
                            Ok(Event { kind: EventKind::Create(_), .. }) |
                            Ok(Event { kind: EventKind::Modify(_), .. }) |
                            Ok(Event { kind: EventKind::Remove(_), .. }) => {
                                println!("\n{}", "Change detected! Regenerating report...".bright_yellow());

                                let parser = match Parser::from_path(dir_path.as_str()) {
                                    Some(reader) => reader,
                                    None => {
                                        panic!("Error reading definitions");
                                    }
                                };

                                error_table(&parser.features);
                            }
                            _ => {}
                        }
                    }
                    Err(e) => println!("Watch error: {:?}", e),
                }
            }
        }
    }
}
