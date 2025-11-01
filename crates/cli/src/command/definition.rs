use crate::formatter::{info, success};
use colored::Colorize;
use crate::parser::Parser;

pub fn search_definition(name: String, path: Option<String>) {
    let dir_path = path.unwrap_or_else(|| "./definitions".to_string());

    let parser = match Parser::from_path(dir_path.as_str()) {
        Some(reader) => reader,
        None => {
            panic!("Error reading definitions");
        }
    };

    search_and_display_definitions(&name, &parser);
}

fn search_and_display_definitions(search_name: &str, parser: &Parser) {
    let mut found_any = false;
    let mut total_matches = 0;
    info(format!("Searching for '{}'", search_name));

    for feature in &parser.features {
        // Search FlowTypes
        for flow_type in &feature.flow_types {
            if flow_type.identifier == search_name {
                total_matches += 1;
                if !found_any {
                    found_any = true;
                }

                info(String::from("Found flow_type:\n"));
                match serde_json::to_string_pretty(flow_type) {
                    Ok(json) => {
                        let mut index = 0;
                        for line in json.lines() {
                            index += 1;
                            println!("{} {}", format!("{index}:"), line.bright_cyan());
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

                info(String::from("Found data_type:\n"));
                match serde_json::to_string_pretty(data_type) {
                    Ok(json) => {
                        let mut index = 0;
                        for line in json.lines() {
                            index += 1;
                            println!("{} {}", format!("{index}:"), line.bright_cyan());
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

                info(String::from("Found runtime_function_definition:\n"));
                match serde_json::to_string_pretty(runtime_func) {
                    Ok(json) => {
                        let mut index = 0;
                        for line in json.lines() {
                            index += 1;
                            println!("{} {}", format!("{index}:"), line.bright_cyan());
                        }
                    }
                    Err(_) => println!("{}", "Error serializing RuntimeFunction".red()),
                }
            }
        }
    }

    if !found_any {
        println!(
            "{}",
            format!("\n{}: {}", "error".red(), "Found no matching definition(s)")
        );
    } else {
        success(format!("Found {total_matches} matching definition(s)"))
    }
}
