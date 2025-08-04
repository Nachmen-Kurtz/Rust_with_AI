use clap::{Parser, Subcommand};
use clap_complete::generate;
use prettytable::Table;
use serde::Deserialize;
use std::io::{self, ErrorKind, Write}; // Import ErrorKind
use std::process::{Command, Stdio};
use std::time::Duration;

// --- Data Structures and Error Handling (Unchanged) ---
#[derive(Deserialize, Debug)]
pub struct Package {
  pub repo: String,
  pub version: String,
  pub status: Option<String>,
  pub repopage: Option<String>,
}

#[derive(Debug)]
pub enum Error {
  Reqwest(reqwest::Error),
  Http(reqwest::StatusCode),
  Io(io::Error),
}

impl From<reqwest::Error> for Error {
  fn from(err: reqwest::Error) -> Self {
    Error::Reqwest(err)
  }
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Self {
    Error::Io(err)
  }
}

// --- Command-Line Interface Definition (Unchanged) ---

/// A CLI tool to query the Repology API, with auto-paging and completion.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, name = "repology-cli")]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
  /// Query for a package and display versions
  Query(QueryArgs),
  /// Generate shell completion scripts
  GenerateCompletion {
    #[arg(value_enum)]
    shell: clap_complete::Shell,
  },
}

#[derive(Parser, Debug)]
struct QueryArgs {
  /// The name of the package to query
  #[arg(required = true)]
  package_name: String,
  /// Optional: Filter results by the repository/distribution name
  #[arg(short, long)]
  distro: Option<String>,
  /// The network timeout in seconds for the API request
  #[arg(long, default_value_t = 10)]
  timeout: u64,
}

// --- Main Application Logic ---

#[tokio::main]
async fn main() {
  let cli = Cli::parse();

  let result = match cli.command {
    Commands::Query(args) => query_packages(args).await,
    Commands::GenerateCompletion { shell } => {
      let mut cmd = <Cli as clap::CommandFactory>::command();
      let bin_name = cmd.get_name().to_string();
      generate(shell, &mut cmd, bin_name, &mut io::stdout());
      Ok(())
    }
  };

  // MODIFIED: Check for the specific BrokenPipe error before printing
  if let Err(Error::Io(e)) = &result {
    if e.kind() == ErrorKind::BrokenPipe {
      // Silently exit, as this is expected when the user quits the pager.
      return;
    }
  }

  if let Err(e) = result {
    eprintln!("\nError: {:?}", e);
  }
}

async fn query_packages(args: QueryArgs) -> Result<(), Error> {
  let request_url = format!("https://repology.org/api/v1/project/{}", args.package_name);
  eprintln!("Fetching data from: {}", request_url);

  let client = reqwest::Client::builder()
    .user_agent("rust-repology-cli/0.5")
    .timeout(Duration::from_secs(args.timeout))
    .build()?;

  let response = client.get(&request_url).send().await?;

  if !response.status().is_success() {
    return Err(Error::Http(response.status()));
  }

  let mut packages: Vec<Package> = response.json().await?;

  if let Some(distro_filter) = &args.distro {
    eprintln!(
      "Filtering by distribution containing: '{}'\n",
      distro_filter
    );
    let filter_lower = distro_filter.to_lowercase();
    packages.retain(|pkg| pkg.repo.to_lowercase().contains(&filter_lower));
  }

  if packages.is_empty() {
    println!(
      "No packages found for '{}' with the specified filters.",
      args.package_name
    );
  } else {
    let mut table = Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_DEFAULT);
    table.set_titles(prettytable::row![
      "Repository",
      "Version",
      "Status",
      "Repository Page"
    ]);

    for pkg in packages {
      table.add_row(prettytable::row![
        pkg.repo,
        pkg.version,
        pkg.status.as_deref().unwrap_or("N/A"),
        pkg.repopage.as_deref().unwrap_or("N/A")
      ]);
    }

    if atty::is(atty::Stream::Stdout) {
      let mut pager = Command::new("less")
        .arg("-R")
        .stdin(Stdio::piped())
        .spawn()?;
      if let Some(mut stdin) = pager.stdin.take() {
        // MODIFIED: We now wrap the write operation in a match to handle the error
        match stdin.write_all(table.to_string().as_bytes()) {
          Ok(_) => {} // Everything wrote successfully
          Err(ref e) if e.kind() == ErrorKind::BrokenPipe => {
            // The pager was closed, this is not an error, just stop writing.
          }
          Err(e) => {
            // A different, real I/O error occurred.
            return Err(Error::Io(e));
          }
        }
      }
      pager.wait()?;
    } else {
      table.printstd();
    }
  }

  Ok(())
}
