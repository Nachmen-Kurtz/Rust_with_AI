use goblin::elf::Elf;
use std::fs::File;
use std::io::Read;
use std::process::Command;

fn main() {
    // Specify the path to the binary file
    let binary_path = "/home/nachmen/.local/bin/wifi_details"; // Replace with actual path

    if let Err(e) = analyze_binary(binary_path) {
        eprintln!("Error analyzing binary: {}", e);
    }
}

fn analyze_binary(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read the binary file
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Parse ELF binary using goblin
    let elf = Elf::parse(&buffer)?;

    // Print basic ELF information
    println!("=== Binary Analysis for {} ===", path);
    println!("Architecture: {:?}", elf.header.e_machine);
    println!(
        "Endianness: {}",
        if elf.header.e_ident[goblin::elf::header::EI_DATA] == goblin::elf::header::ELFDATA2LSB {
            "Little"
        } else {
            "Big"
        }
    );
    println!("ELF Type: {:?}", elf.header.e_type);
    println!(
        "OS ABI: {:?}",
        elf.header.e_ident[goblin::elf::header::EI_OSABI]
    );
    println!("File Version: {}", elf.header.e_version);
    println!("Entry Point: 0x{:x}", elf.header.e_entry);

    // Check for dynamic libraries (dependencies)
    println!("Dynamic Libraries:");
    if elf.libraries.is_empty() {
        println!("  - None");
    } else {
        for lib in &elf.libraries {
            println!("  - {}", lib);
        }
    }

    // Check for symbols to infer compiler and language
    let compiler_info = infer_compiler(&elf, &buffer);
    println!("Compiler: {}", compiler_info.0);
    println!("Likely Programming Language: {}", compiler_info.1);

    // Run the `file` command for additional details
    let file_output = Command::new("file")
        .arg(path)
        .output()
        .map_err(|e| format!("Failed to run file command: {}", e))?;

    let file_info = String::from_utf8_lossy(&file_output.stdout);
    println!("File Command Output: {}", file_info);

    // Check for specific compiler signatures in the binary
    let signatures = check_compiler_signatures(&buffer);
    println!("Detected Compiler Signatures: {:?}", signatures);

    Ok(())
}

fn infer_compiler(elf: &Elf, buffer: &[u8]) -> (String, String) {
    let mut compiler = "Unknown".to_string();
    let mut language = "Unknown".to_string();

    // Check symbol table for compiler-specific symbols
    for sym in elf.syms.iter() {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if name.contains("gcc") || name.contains("__gcc") {
                compiler = "GCC".to_string();
                language = infer_language_from_symbols(&elf.syms, &elf.strtab);
            } else if name.contains("clang") || name.contains("__clang") {
                compiler = "Clang".to_string();
                language = infer_language_from_symbols(&elf.syms, &elf.strtab);
            }
        }
    }

    // Check for specific sections or patterns
    if buffer.windows(4).any(|window| window == b"rust") {
        language = "Rust".to_string();
        compiler = "rustc".to_string();
    }

    (compiler, language)
}

fn infer_language_from_symbols(
    syms: &goblin::elf::sym::Symtab,
    strtab: &goblin::strtab::Strtab,
) -> String {
    for sym in syms.iter() {
        if let Some(name) = strtab.get_at(sym.st_name) {
            if name.contains("std::") || name.contains("cxx") {
                return "C++".to_string();
            } else if name.contains("_go") || name.contains("runtime.main") {
                return "Go".to_string();
            } else if name.contains("Py_") || name.contains("python") {
                return "Python (C API)".to_string();
            }
        }
    }
    "C".to_string() // Default assumption for many ELF binaries
}

fn check_compiler_signatures(buffer: &[u8]) -> Vec<String> {
    let mut signatures = Vec::new();
    if buffer.windows(10).any(|window| window == b"GNU ld") {
        signatures.push("GNU Linker".to_string());
    }
    if buffer.windows(10).any(|window| window == b"clang") {
        signatures.push("Clang".to_string());
    }
    if buffer.windows(10).any(|window| window == b"rustc") {
        signatures.push("Rustc".to_string());
    }
    signatures
}
