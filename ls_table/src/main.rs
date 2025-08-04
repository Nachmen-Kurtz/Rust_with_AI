use chrono::{DateTime, Local};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use tabled::{
  Table,
  settings::{Alignment, Modify, Style, object::Rows},
};

fn main() -> std::io::Result<()> {
  // Read current directory
  let entries = fs::read_dir(".")?;

  // Create a vector to store file information
  let mut files: Vec<FileInfo> = Vec::new();

  // Collect file information, including hidden files
  for entry in entries {
    let entry = entry?;
    let metadata = entry.metadata()?;

    let name = entry.file_name().into_string().unwrap_or_default();
    let size = metadata.len();
    let modified: DateTime<Local> = metadata.modified()?.into();
    let permissions = metadata.permissions();
    let perms = format_permissions(permissions.mode());
    let file_type = if metadata.is_dir() { "dir" } else { "file" };

    files.push(FileInfo {
      name,
      size,
      modified: modified.format("%Y-%m-%d %H:%M:%S").to_string(),
      permissions: perms,
      file_type: file_type.to_string(),
    });
  }

  // Sort files by size (descending)
  files.sort_by(|a, b| b.size.cmp(&a.size));

  // Create table with a different style
  let table = Table::new(files)
    .with(Style::modern())
    .with(Modify::new(Rows::new(1..)).with(Alignment::left()))
    .to_string();

  println!("{}", table);

  Ok(())
}

// Struct to hold file information
#[derive(tabled::Tabled)]
struct FileInfo {
  #[tabled(rename = "Name")]
  name: String,
  #[tabled(rename = "Type")]
  file_type: String,
  #[tabled(rename = "Size (bytes)")]
  size: u64,
  #[tabled(rename = "Modified")]
  modified: String,
  #[tabled(rename = "Permissions")]
  permissions: String,
}

// Format permissions in Unix style (e.g., -rwxr-xr-x)
fn format_permissions(mode: u32) -> String {
  let user = triplet(mode, 6);
  let group = triplet(mode, 3);
  let other = triplet(mode, 0);
  format!("{}{}{}", user, group, other)
}

fn triplet(mode: u32, shift: u32) -> String {
  let r = if (mode >> shift) & 0x4 != 0 { "r" } else { "-" };
  let w = if (mode >> shift) & 0x2 != 0 { "w" } else { "-" };
  let x = if (mode >> shift) & 0x1 != 0 { "x" } else { "-" };
  format!("{}{}{}", r, w, x)
}
