use std::fs;
use std::io;

fn main() -> io::Result<()> {
  println!("🔍 Reading information directly from the Linux kernel...");
  let kernel_version = fs::read_to_string("/proc/version")?;
  println!("\n✅ The kernel version running on the system:");
  println!("{}", kernel_version);
  Ok(())
}
