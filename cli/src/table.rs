use colored::*;
use reader::parser::DefinitionError;
use reader::parser::Feature;
use tabled::{
    Table, Tabled,
    settings::{Modify, Style, Width, object::Columns},
};
use tucana::shared::{DefinitionDataType, FlowType, RuntimeFunctionDefinition};

#[derive(Tabled)]
struct FlowTypeRow {
    #[tabled(rename = "#")]
    index: usize,
    #[tabled(rename = "Identifier")]
    identifier: String,
}

#[derive(Tabled)]
struct DataTypeRow {
    #[tabled(rename = "#")]
    index: usize,
    #[tabled(rename = "Identifier")]
    identifier: String,
}

#[derive(Tabled)]
struct RuntimeFunctionRow {
    #[tabled(rename = "#")]
    index: usize,
    #[tabled(rename = "Runtime Name")]
    runtime_name: String,
}

#[derive(Tabled)]
struct ErrorRow {
    #[tabled(rename = "#")]
    index: usize,
    #[tabled(rename = "Type")]
    definition_type: String,
    #[tabled(rename = "Definition")]
    definition: String,
    #[tabled(rename = "Error")]
    error: String,
}

#[derive(Tabled)]
struct FeatureSummaryRow {
    #[tabled(rename = "Feature")]
    feature_name: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Errors")]
    error_count: usize,
    #[tabled(rename = "Flow Types")]
    flow_types: usize,
    #[tabled(rename = "Data Types")]
    data_types: usize,
    #[tabled(rename = "Runtime Functions")]
    runtime_functions: usize,
}

#[derive(Tabled)]
struct GeneralErrorRow {
    #[tabled(rename = "#")]
    index: usize,
    #[tabled(rename = "Feature")]
    feature_name: String,
    #[tabled(rename = "Type")]
    definition_type: String,
    #[tabled(rename = "Definition")]
    definition: String,
    #[tabled(rename = "Error")]
    error: String,
}

pub fn feature_table(feature: &Feature) {
    // Header
    println!(
        "\n{}",
        "╔══════════════════════════════════════════════════════════════════════════════╗"
            .bright_cyan()
    );
    println!(
        "{} {} {}",
        "║".bright_cyan(),
        format!("FEATURE REPORT: {}", feature.name)
            .bright_white()
            .bold()
            .on_blue(),
        "║".bright_cyan()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════════════════════════╝"
            .bright_cyan()
    );

    // Flow Types Section
    println!(
        "\n{}",
        format!("FLOW TYPES ({} defined)", feature.flow_types.len())
            .bright_blue()
            .bold()
    );
    if !feature.flow_types.is_empty() {
        let flow_type_rows: Vec<FlowTypeRow> = feature
            .flow_types
            .iter()
            .enumerate()
            .map(|(i, FlowType { identifier, .. })| FlowTypeRow {
                index: i + 1,
                identifier: identifier.clone(),
            })
            .collect();

        let table = Table::new(flow_type_rows)
            .with(Style::rounded())
            .to_string();
        println!("{}", table.bright_green());
    } else {
        println!("{}", "  No flow types defined.".dimmed());
    }

    // Data Types Section
    println!(
        "\n{}",
        format!("DATA TYPES ({} defined)", feature.data_types.len())
            .bright_blue()
            .bold()
    );
    if !feature.data_types.is_empty() {
        let data_type_rows: Vec<DataTypeRow> = feature
            .data_types
            .iter()
            .enumerate()
            .map(|(i, DefinitionDataType { identifier, .. })| DataTypeRow {
                index: i + 1,
                identifier: identifier.clone(),
            })
            .collect();

        let table = Table::new(data_type_rows)
            .with(Style::rounded())
            .to_string();
        println!("{}", table.bright_green());
    } else {
        println!("{}", "  No data types defined.".dimmed());
    }

    // Runtime Functions Section
    println!(
        "\n{}",
        format!(
            "RUNTIME FUNCTIONS ({} defined)",
            feature.runtime_functions.len()
        )
        .bright_blue()
        .bold()
    );
    if !feature.runtime_functions.is_empty() {
        let runtime_function_rows: Vec<RuntimeFunctionRow> = feature
            .runtime_functions
            .iter()
            .enumerate()
            .map(
                |(i, RuntimeFunctionDefinition { runtime_name, .. })| RuntimeFunctionRow {
                    index: i + 1,
                    runtime_name: runtime_name.clone(),
                },
            )
            .collect();

        let table = Table::new(runtime_function_rows)
            .with(Style::rounded())
            .to_string();
        println!("{}", table.bright_green());
    } else {
        println!("{}", "  No runtime functions defined.".dimmed());
    }

    // Errors Section
    println!(
        "\n{}",
        format!("DEFINITION ERRORS ({} found)", feature.errors.len())
            .bright_red()
            .bold()
    );
    if !feature.errors.is_empty() {
        let error_rows: Vec<ErrorRow> = feature
            .errors
            .iter()
            .enumerate()
            .map(
                |(
                    i,
                    DefinitionError {
                        definition,
                        definition_type,
                        error,
                    },
                )| ErrorRow {
                    index: i + 1,
                    definition_type: format!("{}", definition_type),
                    definition: definition.clone(),
                    error: error.clone(),
                },
            )
            .collect();

        let table = Table::new(error_rows)
            .with(Style::rounded())
            .with(Modify::new(Columns::single(0)).with(Width::wrap(5))) // Index column
            .with(Modify::new(Columns::single(1)).with(Width::wrap(15))) // Type column
            .with(Modify::new(Columns::single(2)).with(Width::wrap(20))) // Definition column
            .with(Modify::new(Columns::single(3)).with(Width::wrap(40))) // Error column
            .to_string();
        println!("{}", table.bright_red());
    } else {
        println!("{}", "  No errors found!".bright_green());
    }

    println!("\n{}", "═".repeat(80).bright_cyan());
}

pub fn error_table(features: &Vec<Feature>) {
    println!(
        "\n{}",
        "╔══════════════════════════════════════════════════════════════════════════════╗"
            .bright_cyan()
    );
    println!(
        "{} {} {}",
        "║".bright_cyan(),
        "ERRORS".bright_white().bold().on_blue(),
        "║".bright_cyan()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════════════════════════╝"
            .bright_cyan()
    );

    // Collect all errors from all features
    let mut all_errors = Vec::new();
    for feature in features {
        for error in &feature.errors {
            all_errors.push((feature.name.clone(), error));
        }
    }

    // Display all errors table
    println!(
        "\n{}",
        format!(
            "ALL DEFINITION ERRORS ({} found across {} features)",
            all_errors.len(),
            features.len()
        )
        .bright_red()
        .bold()
    );

    if !all_errors.is_empty() {
        let error_rows: Vec<GeneralErrorRow> = all_errors
            .iter()
            .enumerate()
            .map(|(i, (feature_name, error))| GeneralErrorRow {
                index: i + 1,
                feature_name: feature_name.clone(),
                definition_type: format!("{}", error.definition_type),
                definition: error.definition.clone(),
                error: error.error.clone(),
            })
            .collect();

        let table = Table::new(error_rows)
            .with(Style::rounded())
            .with(Modify::new(Columns::single(0)).with(Width::wrap(5))) // Index column
            .with(Modify::new(Columns::single(1)).with(Width::wrap(15))) // Feature column
            .with(Modify::new(Columns::single(2)).with(Width::wrap(12))) // Type column
            .with(Modify::new(Columns::single(3)).with(Width::wrap(18))) // Definition column
            .with(Modify::new(Columns::single(4)).with(Width::wrap(35))) // Error column
            .to_string();
        println!("{}", table.bright_red());
    } else {
        println!(
            "{}",
            "  No errors found across all features!".bright_green()
        );
    }
}

pub fn summary_table(features: &Vec<Feature>) {
    println!(
        "\n{}",
        "╔══════════════════════════════════════════════════════════════════════════════╗"
            .bright_cyan()
    );
    println!(
        "{} {} {}",
        "║".bright_cyan(),
        "CONCLUSION".bright_white().bold().on_blue(),
        "║".bright_cyan()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════════════════════════╝"
            .bright_cyan()
    );

    // Create summary table
    let summary_rows: Vec<FeatureSummaryRow> = features
        .iter()
        .map(|feature| {
            let is_successful = feature.errors.is_empty();
            let status = if is_successful {
                "✅ SUCCESS".to_string()
            } else {
                "❌ FAILED".to_string()
            };

            FeatureSummaryRow {
                feature_name: feature.name.clone(),
                status,
                error_count: feature.errors.len(),
                flow_types: feature.flow_types.len(),
                data_types: feature.data_types.len(),
                runtime_functions: feature.runtime_functions.len(),
            }
        })
        .collect();

    if !summary_rows.is_empty() {
        let table = Table::new(summary_rows)
            .with(Style::rounded())
            .with(Modify::new(Columns::single(0)).with(Width::wrap(20))) // Feature name
            .with(Modify::new(Columns::single(1)).with(Width::wrap(12))) // Status
            .with(Modify::new(Columns::single(2)).with(Width::wrap(8))) // Errors
            .with(Modify::new(Columns::single(3)).with(Width::wrap(12))) // Flow Types
            .with(Modify::new(Columns::single(4)).with(Width::wrap(12))) // Data Types
            .with(Modify::new(Columns::single(5)).with(Width::wrap(18))) // Runtime Functions
            .to_string();

        println!("{}", table.bright_blue());
    }

    // Overall success assessment
    let total_errors: usize = features.iter().map(|f| f.errors.len()).sum();
    let total_features = features.len();
    let successful_features = features.iter().filter(|f| f.errors.is_empty()).count();

    println!("\n{}", "OVERALL SUMMARY".bright_blue().bold());

    if total_errors == 0 {
        println!(
            "{}",
            format!(
                "PROCESS SUCCESSFUL! All {} feature(s) processed without errors.",
                total_features
            )
            .bright_green()
            .bold()
        );
    } else {
        println!(
            "{}",
            format!(
                "PROCESS FAILED! {} error(s) found across {} feature(s).",
                total_errors, total_features
            )
            .bright_red()
            .bold()
        );
        println!(
            "   {} {} successful, {} {} failed",
            successful_features.to_string().bright_green(),
            if successful_features == 1 {
                "feature"
            } else {
                "features"
            },
            (total_features - successful_features)
                .to_string()
                .bright_red(),
            if (total_features - successful_features) == 1 {
                "feature"
            } else {
                "features"
            }
        );
    }

    println!("\n{}", "═".repeat(80).bright_cyan());
}
