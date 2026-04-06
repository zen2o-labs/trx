use trx_engine::ast::{NamedDiagram, Project};
use trx_engine::layout::engine::LayoutEngine;
use trx_engine::layout::parse_and_render;
use trx_engine::render::RenderEngine;

struct MockLayout;
impl LayoutEngine for MockLayout {
    fn layout(&mut self, _diagram: &mut NamedDiagram, _bounds: kurbo::Rect) {}
    fn configure(&mut self, _: &std::collections::HashMap<String, f64>) {}
}

struct MockRender;
impl RenderEngine for MockRender {
    fn render(&self, _project: &Project) -> Vec<u8> {
        b"<svg></svg>".to_vec()
    }
}

#[test]
fn test_parse_render() {
    let code = "A[Service] -> B[DB]";

    let mut layout_engine = MockLayout;
    let mut render_engine = MockRender;

    let svg_bytes = parse_and_render(code, &mut layout_engine, &mut render_engine);
    assert!(!svg_bytes.is_empty());
}

#[test]
fn test_math_torture() {
    let mut code = String::new();
    code.push_str("let v0 = 1.0\n");
    for i in 1..=100 {
        let prev = i - 1;
        code.push_str(&format!(
            "let v{} = Math.round(v{} * 1.05 + Math.sin(v{}) - Math.cos(v{}))\n",
            i, prev, prev, prev
        ));
    }
    code.push_str("# MathTorture\n");
    code.push_str("  layer G1 {\n");
    for i in 2..=10 {
        code.push_str(&format!("    layer G{} {{\n", i));
    }
    code.push_str("      N0 { width: v100 }\n");
    for _ in 2..=10 {
        code.push_str("    }\n");
    }
    code.push_str("  }\n");

    let mut project = trx_engine::parser::parse(&code).expect("Parse failed");
    trx_engine::evaluator::evaluate_project(&mut project).expect("Eval failed");

    let mut current_layer = &project.diagrams[0].root.layers[0];
    for _ in 2..=10 {
        current_layer = &current_layer.layers[0];
    }
    let last_val = current_layer.nodes[0].width as f64;

    let mut expected = 1.0f64;
    for _ in 1..=100 {
        expected = libm::round(expected * 1.05 + libm::sin(expected) - libm::cos(expected));
    }

    // precision allows minimal cast delta
    assert!(
        (last_val - expected).abs() < 1e-4,
        "Math determinism check failed! Got: {}, Expected: {}",
        last_val,
        expected
    );
}
