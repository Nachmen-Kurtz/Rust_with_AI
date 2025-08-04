use std::env;
use std::fs::{self, DirEntry};
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

// Function to format file permissions in Unix style (e.g., -rwxr-xr-x)
fn format_permissions(perms: &fs::Permissions) -> String {
  let mode = perms.mode();
  let user = [
    (mode & 0o400, 'r'),
    (mode & 0o200, 'w'),
    (mode & 0o100, 'x'),
  ];
  let group = [
    (mode & 0o040, 'r'),
    (mode & 0o020, 'w'),
    (mode & 0o010, 'x'),
  ];
  let other = [
    (mode & 0o004, 'r'),
    (mode & 0o002, 'w'),
    (mode & 0o001, 'x'),
  ];

  let mut result = String::new();
  result.push(if perms.mode() & 0o100000 != 0 {
    '-'
  } else {
    'd'
  }); // File or directory
  for &(bit, ch) in user.iter().chain(group.iter()).chain(other.iter()) {
    result.push(if bit != 0 { ch } else { '-' });
  }
  result
}

// Function to process and display a single directory entry
fn process_entry(entry: io::Result<DirEntry>) -> io::Result<()> {
  let entry = entry?;
  let path = entry.path();
  let metadata = entry.metadata()?;

  // Get file type, permissions, and size
  let file_type = if metadata.is_dir() { "dir" } else { "file" };
  let permissions = format_permissions(&metadata.permissions());
  let size = metadata.len();
  let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("???");

  // Print formatted output
  println!("{} {:>10} {} {}", permissions, size, file_type, file_name);
  Ok(())
}

fn main() -> io::Result<()> {
  // Get directory from command-line argument or use current directory
  let dir_path = env::args().nth(1).unwrap_or_else(|| ".".to_string());
  let path = Path::new(&dir_path);

  // Check if the path is a directory
  if !path.is_dir() {
    eprintln!("Error: '{}' is not a directory", dir_path);
    std::process::exit(1);
  }

  // Read directory entries and process them
  for entry in fs::read_dir(path)? {
    if let Err(e) = process_entry(entry) {
      eprintln!("Error processing entry: {}", e);
    }
  }

  Ok(())
}
