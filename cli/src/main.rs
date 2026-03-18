use std::env;
use std::fs;
use std::path::Path;
use trx_engine::parser::parse;
use trx_layout::apply_layout;
use std::io::{self, Write};

const LOGO: &[u8] = include_bytes!("../../../assets/logo.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout().lock();
    let _ = stdout.write_all(LOGO);
    let _ = stdout.flush();

    let args: Vec<String> = env::args().collect();
    if args.len() < 4 || args[1] != "compile" {
        eprintln!("Usage: trx compile <input.trx> <output.json>");
        std::process::exit(1);
    }

    let input_path = &args[2];
    let output_path = &args[3];
    
    let path = Path::new(input_path);
    if !path.exists() {
        eprintln!("Error: File '{}' not found.", input_path);
        std::process::exit(1);
    }

    let input = fs::read_to_string(input_path)?;
    println!("Reading TRX project from {}...", input_path);

    let mut project = parse(&input).map_err(|e| e)?;
    println!("Found {} diagrams.", project.diagrams.len());

    println!("Calculating layouts...");
    apply_layout(&mut project);

    let json_output = serde_json::to_string_pretty(&project)?;
    fs::write(output_path, json_output)?;
    println!("Generated JSON: {}", output_path);

    println!("Success!");
    Ok(())
}