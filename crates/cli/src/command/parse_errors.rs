use crate::formatter::{error, error_without_trace};
use crate::parser::Parser;

pub fn fail_on_parser_errors(parser: &Parser) {
    let mut total_errors = 0usize;

    for module in &parser.modules {
        for parse_error in &module.errors {
            total_errors += 1;
            println!(
                "{}",
                error(
                    format!(
                        "Failed to parse {} `{}`: {}",
                        parse_error.definition_type, parse_error.definition, parse_error.error
                    ),
                    &parse_error.path,
                )
            );
        }
    }

    if total_errors > 0 {
        error_without_trace(format!(
            "Found {} definition parse error(s). Fix the files above and retry.",
            total_errors
        ));
        std::process::exit(1);
    }
}
