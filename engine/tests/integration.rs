use trx_engine::layout::parse_and_render;
use trx_engine::layout::engine::LayoutEngine;
use trx_engine::render::RenderEngine;
use trx_engine::ast::{Project, NamedDiagram};

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