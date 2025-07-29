use crate::table::*;
use clap::{Parser as ClapParser, Subcommand};
use code0_definition_reader::parser::Parser;
use colored::*;
use notify::{Event, EventKind, RecursiveMode, Watcher, recommended_watcher};
use std::sync::mpsc::channel;

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
        name: Option<String>,
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
            let dir_path = path.unwrap_or_else(|| "./definitions".to_string());

            let parser = match Parser::from_path(dir_path.as_str()) {
                Some(reader) => reader,
                None => {
                    panic!("Error reading definitions");
                }
            };
            error_table(&parser.features);
            summary_table(&parser.features);
        }
        Commands::Feature { name, path } => {
            let dir_path = path.unwrap_or_else(|| "./definitions".to_string());

            let parser = match Parser::from_path(dir_path.as_str()) {
                Some(reader) => reader,
                None => {
                    panic!("Error reading definitions");
                }
            };

            if let Some(feature_name) = name {
                let mut features_to_report = Vec::new();
                for feature in &parser.features {
                    if feature.name == feature_name {
                        feature_table(feature);
                        features_to_report.push(feature.clone());
                    }
                }
                summary_table(&features_to_report);
            } else {
                for feature in &parser.features {
                    feature_table(feature);
                }
                summary_table(&parser.features);
            }
        }
        Commands::Definition { name, path } => {
            let dir_path = path.unwrap_or_else(|| "./definitions".to_string());

            let parser = match Parser::from_path(dir_path.as_str()) {
                Some(reader) => reader,
                None => {
                    panic!("Error reading definitions");
                }
            };

            search_and_display_definitions(&name, &parser);
        }
        Commands::Watch { path } => {
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
    }
}

fn search_and_display_definitions(search_name: &str, parser: &Parser) {
    let mut found_any = false;
    let mut total_matches = 0;
    println!(
        "{}",
        format!("Searching for definitions matching: '{search_name}'")
            .bright_yellow()
            .bold()
    );
    println!("{}", "─".repeat(60).dimmed());

    for feature in &parser.features {
        // Search FlowTypes
        for flow_type in &feature.flow_types {
            if flow_type.identifier == search_name {
                total_matches += 1;
                if !found_any {
                    found_any = true;
                }

                println!("\n{}", "FlowType".bright_cyan().bold());
                match serde_json::to_string_pretty(flow_type) {
                    Ok(json) => {
                        for line in json.lines() {
                            println!("{}", line.bright_green());
                        }
                    }
                    Err(_) => println!("{}", "Error serializing FlowType".red()),
                }
            }
        }

        // Search DataTypes
        for data_type in &feature.data_types {
            if data_type.identifier == search_name {
                total_matches += 1;
                if !found_any {
                    found_any = true;
                }

                println!("\n{}", "DataType".bright_cyan().bold());
                match serde_json::to_string_pretty(data_type) {
                    Ok(json) => {
                        for line in json.lines() {
                            println!("{}", line.bright_green());
                        }
                    }
                    Err(_) => println!("{}", "Error serializing DataType".red()),
                }
            }
        }

        // Search RuntimeFunctions
        for runtime_func in &feature.runtime_functions {
            if runtime_func.runtime_name == search_name {
                total_matches += 1;
                if !found_any {
                    found_any = true;
                }

                println!("\n{}", "RuntimeFunction".bright_cyan().bold());
                match serde_json::to_string_pretty(runtime_func) {
                    Ok(json) => {
                        let mut index = 0;
                        for line in json.lines() {
                            index += 1;
                            println!(
                                "{} {}",
                                format!("{index}:").bright_blue(),
                                line.bright_green()
                            );
                        }
                    }
                    Err(_) => println!("{}", "Error serializing RuntimeFunction".red()),
                }
            }
        }
    }

    if !found_any {
        println!(
            "\n{}",
            format!("No definitions found matching '{search_name}'")
                .red()
                .bold()
        );
    } else {
        println!("\n{}", "─".repeat(60).dimmed());
        println!(
            "{}",
            format!("Found {total_matches} matching definition(s)").bright_yellow()
        );
    }
}
