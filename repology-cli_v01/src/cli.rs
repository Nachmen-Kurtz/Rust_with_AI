use clap::{Parser, Subcommand};

/// A CLI tool to query the Repology API, with auto-paging and completion.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, name = "repology-cli")]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
  /// Query for a package and display versions
  #[command(name = "-q")]
  Q(QueryArgs),
  /// Generate shell completion scripts
  GenerateCompletion {
    #[arg(value_enum)]
    shell: clap_complete::Shell,
  },
}

#[derive(Parser, Debug)]
pub struct QueryArgs {
  /// The name of the package to query
  #[arg(required = true)]
  pub package_name: String,
  /// Optional: Filter results by the repository/distribution name
  #[arg(short, long)]
  pub distro: Option<String>,
  /// The network timeout in seconds for the API request
  #[arg(long, default_value_t = 10)]
  pub timeout: u64,
}
