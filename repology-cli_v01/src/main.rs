use clap::Parser;
use clap_complete::generate;
use std::io::{self, ErrorKind};

mod cli;
mod models;
mod query;

use cli::Cli;
use models::Error;
use query::query_packages;

#[tokio::main]
async fn main() {
  let cli = Cli::parse();

  let result = match cli.command {
    cli::Commands::Q(args) => query_packages(args).await,
    cli::Commands::GenerateCompletion { shell } => {
      let mut cmd = <Cli as clap::CommandFactory>::command();
      let bin_name = cmd.get_name().to_string();
      generate(shell, &mut cmd, bin_name, &mut io::stdout());
      Ok(())
    }
  };

  if let Err(Error::Io(e)) = &result {
    if e.kind() == ErrorKind::BrokenPipe {
      return;
    }
  }

  if let Err(e) = result {
    match e {
      Error::Reqwest(err) => eprintln!("\nNetwork error: {}", err),
      Error::Http(status) => eprintln!("\nHTTP error: Status code {}", status),
      Error::Io(err) => eprintln!("\nI/O error: {}", err),
    }
  }
}
