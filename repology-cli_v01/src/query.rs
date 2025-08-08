use crate::models::{Error, Package};
use prettytable::{Table, format};
use std::io::{ErrorKind, Write};
use std::process::{Command, Stdio};
use std::time::Duration;

use crate::cli::QueryArgs;

pub async fn query_packages(args: QueryArgs) -> Result<(), Error> {
  let request_url = format!("https://repology.org/api/v1/project/{}", args.package_name);

  let client = reqwest::Client::builder()
    .user_agent("rust-repology-cli/0.5")
    .timeout(Duration::from_secs(args.timeout))
    .build()?;

  let response = client.get(&request_url).send().await?;

  if !response.status().is_success() {
    return Err(Error::Http(response.status()));
  }

  let packages: Vec<Package> = response.json().await?;

  let mut filtered_packages = packages;
  if let Some(distro_filter) = &args.distro {
    let filter_lower = distro_filter.to_lowercase();
    filtered_packages.retain(|pkg| pkg.repo.to_lowercase().contains(&filter_lower));
  }

  if filtered_packages.is_empty() {
    println!(
      "No packages found for '{}' with the specified filters.",
      args.package_name
    );
  } else {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_DEFAULT);
    table.set_titles(prettytable::row!["Repo", "Version", "Status"]);

    for pkg in filtered_packages.iter() {
      table.add_row(prettytable::row![
        &pkg.repo,
        &pkg.version,
        pkg.status.as_deref().unwrap_or("N/A")
      ]);
    }

    if atty::is(atty::Stream::Stdout) {
      let mut pager = Command::new("less")
        .arg("-R")
        .stdin(Stdio::piped())
        .spawn()?;
      if let Some(mut stdin) = pager.stdin.take() {
        match stdin.write_all(table.to_string().as_bytes()) {
          Ok(_) => {}
          Err(ref e) if e.kind() == ErrorKind::BrokenPipe => {}
          Err(e) => {
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
