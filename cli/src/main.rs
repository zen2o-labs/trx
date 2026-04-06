use serde_json::json;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use trx_engine::evaluator::evaluate_project;
use trx_engine::parser::error::ParseError;
use trx_engine::parser::parse;
use trx_engine::render::svg::render_svg;
use trx_layout::apply_layout;

const LOGO: &[u8] = include_bytes!("../../assets/TextTRX.txt");

#[derive(Debug, Clone)]
pub struct SourceSegment {
    pub file_path: String,
    pub flattened_start_line: usize,
    pub flattened_end_line: usize,
    pub original_start_line: usize,
}

pub struct PreprocessorResult {
    pub flattened_code: String,
    pub source_map: Vec<SourceSegment>,
}

fn resolve_trx_imports(base_path: &Path, content: &str) -> io::Result<PreprocessorResult> {
    let mut flattened_code = String::new();
    let mut source_map = Vec::new();
    let mut current_flattened_line = 1;
    let mut current_original_line = 1;
    let mut current_segment_start = 1;

    let file_path_str = base_path.to_string_lossy().to_string();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("include ") {
            if let Some(start) = trimmed.find('"') {
                if let Some(end) = trimmed[start + 1..].find('"') {
                    if current_flattened_line > current_segment_start {
                        source_map.push(SourceSegment {
                            file_path: file_path_str.clone(),
                            flattened_start_line: current_segment_start,
                            flattened_end_line: current_flattened_line - 1,
                            original_start_line: current_original_line
                                - (current_flattened_line - current_segment_start),
                        });
                    }

                    let rel_path = &trimmed[start + 1..start + 1 + end];
                    let inc_path = base_path.parent().unwrap_or(Path::new("")).join(rel_path);

                    let inc_content = fs::read_to_string(&inc_path)?;
                    let inc_result = resolve_trx_imports(&inc_path, &inc_content)?;

                    for mut seg in inc_result.source_map {
                        let len = seg.flattened_end_line - seg.flattened_start_line;
                        seg.flattened_start_line += current_flattened_line - 1;
                        seg.flattened_end_line = seg.flattened_start_line + len;
                        source_map.push(seg);
                    }

                    flattened_code.push_str(&inc_result.flattened_code);
                    if !flattened_code.ends_with('\n') {
                        flattened_code.push('\n');
                    }

                    let lines_added = inc_result.flattened_code.lines().count();
                    current_flattened_line += lines_added;
                    current_segment_start = current_flattened_line;
                }
            }
            current_original_line += 1;
        } else {
            flattened_code.push_str(line);
            flattened_code.push('\n');
            current_flattened_line += 1;
            current_original_line += 1;
        }
    }

    if current_flattened_line > current_segment_start {
        source_map.push(SourceSegment {
            file_path: file_path_str.clone(),
            flattened_start_line: current_segment_start,
            flattened_end_line: current_flattened_line - 1,
            original_start_line: current_original_line
                - (current_flattened_line - current_segment_start),
        });
    }

    Ok(PreprocessorResult {
        flattened_code,
        source_map,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout().lock();
    let _ = stdout.write_all(LOGO);
    let _ = stdout.flush();

    let args: Vec<String> = env::args().collect();
    if args.len() < 4 || args[1] != "compile" {
        eprintln!("Usage: trx compile <input.trx> <output.json|svg>");
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

    let preprocessed = resolve_trx_imports(&path, &input)?;

    let mut project = match parse(&preprocessed.flattened_code) {
        Ok(p) => p,
        Err(e) => {
            let (mut err_msg, mut err_line, mut err_col) = ("".to_string(), 0, 0);
            if let ParseError::ParseFailed { location, message } = &e {
                err_msg = message.clone();
                err_line = location.line;
                err_col = location.col;
            } else {
                err_msg = e.to_string();
            }

            let mut real_file = input_path.to_string();
            let mut real_line = err_line;

            for seg in &preprocessed.source_map {
                if err_line >= seg.flattened_start_line && err_line <= seg.flattened_end_line {
                    real_line = seg.original_start_line + (err_line - seg.flattened_start_line);
                    real_file = seg.file_path.clone();
                    break;
                }
            }

            let diag = json!({
                "error": err_msg,
                "line": real_line,
                "col": err_col,
                "file": real_file
            });

            eprintln!("{}", serde_json::to_string_pretty(&diag).unwrap());
            std::process::exit(1);
        }
    };
    println!("Found {} diagrams.", project.diagrams.len());

    println!("Evaluating math expressions...");
    if let Err(e) = evaluate_project(&mut project) {
        let diag = json!({
            "error": e.to_string(),
            "line": 0,
            "col": 0,
            "file": input_path
        });
        eprintln!("{}", serde_json::to_string_pretty(&diag).unwrap());
        std::process::exit(1);
    }

    println!("Calculating layouts...");
    apply_layout(&mut project);

    if output_path.ends_with(".svg") {
        let svg_output = render_svg(&project);
        fs::write(output_path, svg_output)?;
        println!("Generated SVG: {}", output_path);
    } else {
        let json_output = serde_json::to_string_pretty(&project)?;
        fs::write(output_path, json_output)?;
        println!("Generated JSON: {}", output_path);
    }

    println!("Success!");
    Ok(())
}
