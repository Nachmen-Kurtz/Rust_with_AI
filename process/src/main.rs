use std::fs;
use std::path::Path;
// Import the necessary components from the prettytable library
use prettytable::{Table, Row, Cell, format};

fn main() {
    // 1. Create a new table object
    let mut table = Table::new();
    
    // 2. Adding the header row to the table
    table.add_row(Row::new(vec![
        Cell::new("PID").style_spec("bFg"), // b = bold, Fg = a(uto) green
        Cell::new("Name").style_spec("bFb"), // b = bold, Fb = a(uto) blue
        Cell::new("State").style_spec("bFr"), // b = bold, Fr = a(uto) red
    ]));

    // Read all records in the /proc folder
    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();

            if path.is_dir() {
                // Check if the folder name is a number (i.e., PID)
                if let Some(dir_name_str) = path.file_name().and_then(|s| s.to_str()) {
                    if dir_name_str.chars().all(char::is_numeric) {
                        // 3. Getting the information about the process and adding it to the table
                        if let Some((pid, name, state)) = get_process_info(&path) {
                            table.add_row(Row::new(vec![
                                Cell::new(&pid),
                                Cell::new(&name),
                                Cell::new(&state),
                            ]));
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("Error: Could not read /proc directory.");
        return;
    }
    
    // 4. Print the entire table to the screen
    table.printstd();
}

/// A function that receives a path to a process and returns its details
/// It returns Option - that is, it will either return the information, or nothing (in case of an error)
fn get_process_info(proc_path: &Path) -> Option<(String, String, String)> {
    let stat_path = proc_path.join("stat");
    let stat_content = fs::read_to_string(stat_path).ok()?; // The ? will return None if the read failed

    let fields: Vec<&str> = stat_content.split_whitespace().collect();

    if fields.len() > 2 {
        let pid = fields[0].to_string();
        let name = fields[1].trim_matches(|c| c == '(' || c == ')').to_string();
        let state = fields[2].to_string();
        Some((pid, name, state))
    } else {
        None // If the format is invalid, we will return None
    }
}
