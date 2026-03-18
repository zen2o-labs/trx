use crate::ast::Project;
use crate::evaluator::evaluate_project;
use crate::parser::parse;
use trx_layout::apply_layout;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmProject {
    inner: Project,
}

#[wasm_bindgen]
impl WasmProject {
    #[wasm_bindgen(constructor)]
    pub fn new(input: &str) -> Result<WasmProject, String> {
        let mut project = parse(input).map_err(|e| e.to_string())?;

        // Evaluation step
        evaluate_project(&mut project);

        // Layer-Aware Layout
        apply_layout(&mut project);

        Ok(WasmProject { inner: project })
    }

    #[wasm_bindgen]
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(&self.inner).map_err(|e| e.to_string())
    }
}
