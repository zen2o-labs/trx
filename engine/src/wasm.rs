use crate::ast::Project;
use crate::evaluator::evaluate_project;
use crate::parser::parse;
use crate::render::svg::render_svg;
use trx_layout::apply_layout;
use wasm_bindgen::prelude::*;

// Scenario-aware WasmProject ──────────────────────

#[wasm_bindgen]
pub struct WasmProject {
    inner: Project,
}

#[wasm_bindgen]
impl WasmProject {
    /// Create a WasmProject from a DSL string.
    /// Optionally filter to a named scenario (Milestone 08).
    #[wasm_bindgen(constructor)]
    pub fn new(input: &str, scenario: Option<String>) -> Result<WasmProject, String> {
        console_error_panic_hook::set_once();

        let mut project = parse(input).map_err(|e| {
            if let crate::parser::error::ParseError::ParseFailed { location, message } = e {
                let diag = serde_json::json!({
                    "error": message,
                    "line": location.line,
                    "col": location.col
                });
                diag.to_string()
            } else {
                let diag = serde_json::json!({
                    "error": e.to_string(),
                    "line": 0,
                    "col": 0
                });
                diag.to_string()
            }
        })?;

        // Filter diagrams to the requested scenario if provided
        if let Some(ref s) = scenario {
            project
                .diagrams
                .retain(|d| d.scenario.as_deref().map(|sc| sc == s).unwrap_or(true));
        }

        evaluate_project(&mut project).map_err(|e| {
            let diag = serde_json::json!({
                "error": e.to_string(),
                "line": 0,
                "col": 0
            });
            diag.to_string()
        })?;
        apply_layout(&mut project);

        Ok(WasmProject { inner: project })
    }

    #[wasm_bindgen]
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(&self.inner).map_err(|e| e.to_string())
    }

    #[wasm_bindgen]
    pub fn to_svg(&self) -> String {
        render_svg(&self.inner)
    }
}

// Zero-Copy Pointer Bridge

/// Allocate `size` bytes of Wasm linear memory and return a pointer.
/// The JS host writes DSL text here before calling `render_svg_from_ptr`.
#[wasm_bindgen]
pub fn alloc(size: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

/// Render TRX DSL stored at `ptr` (with byte length `len`) directly to an SVG string.
/// This avoids any JS↔Wasm string copying overhead.
///
/// # Safety
/// Caller must ensure `ptr` is a valid pointer to `len` bytes of UTF-8 text
/// previously allocated via `alloc()`.
#[wasm_bindgen]
pub unsafe fn render_svg_from_ptr(ptr: *mut u8, len: usize) -> String {
    let slice = std::slice::from_raw_parts(ptr, len);
    let input = match std::str::from_utf8(slice) {
        Ok(s) => s,
        Err(_) => return String::from("<svg><text>Invalid UTF-8 input</text></svg>"),
    };

    let mut project = match parse(input) {
        Ok(p) => p,
        Err(e) => {
            let error_msg =
                if let crate::parser::error::ParseError::ParseFailed { location, message } = e {
                    format!("Line {}:{}: {}", location.line, location.col, message)
                } else {
                    e.to_string()
                };
            return format!("<svg><text>Parse error: {}</text></svg>", error_msg);
        }
    };

    if let Err(e) = evaluate_project(&mut project) {
        return format!("<svg><text>Evaluation error: {}</text></svg>", e);
    }
    apply_layout(&mut project);
    render_svg(&project)
}

/// Free memory previously allocated by `alloc()`.
///
/// # Safety
/// Caller must pass the exact `ptr` and `len` returned by `alloc()`.
#[wasm_bindgen]
pub unsafe fn dealloc(ptr: *mut u8, len: usize) {
    let _ = Vec::from_raw_parts(ptr, 0, len);
}
