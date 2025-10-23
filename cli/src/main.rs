use clap::{Parser as ClapParser, Subcommand};

mod analyser;
mod command;
mod formatter;
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
    Download {
        #[arg(short, long)]
        tag: Option<String>,
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        features: Option<Vec<String>>,
    },
    Bundle {
        #[arg(short, long)]
        path: Option<String>,
        #[arg(short, long)]
        out: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Bundle { path, out } => command::bundle::bundle(path, out),
        Commands::Report { path } => command::report::report_errors(path),
        Commands::Feature { name, path } => command::feature::search_feature(name, path),
        Commands::Definition { name, path } => command::definition::search_definition(name, path),
        Commands::Download { tag, features } => {
            command::download::handle_download(tag, features).await
        }
        Commands::Watch { path } => command::watch::watch_for_changes(path).await,
    }
}
