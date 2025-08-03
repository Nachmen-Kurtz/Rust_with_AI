use std::fs;

fn main() -> std::io::Result<()> {
    let _ = fs::write("test.txt", "Helo")?;
    println!("Success");

    let content = fs::read_to_string("test.txt")?;
    println!("the content: {}", content);
    Ok(())
}
