use crate::ast::{Layer, NamedDiagram, Node, ShapeKind};
use crate::ast::style_buffer::StyleBuffer;
use std::collections::HashMap;
use std::f32::consts::PI;

fn collect_nodes<'a>(layer: &'a Layer, map: &mut HashMap<&'a str, &'a Node>) {
    for node in &layer.nodes {
        map.insert(&node.id, node);
    }
    for child in &layer.layers {
        collect_nodes(child, map);
    }
}

pub fn render_diagrams(diagrams: &[NamedDiagram], svg: &mut String, y_offset: &mut f32) {
    for diagram in diagrams {
        svg.push_str(&format!(
            r#"<text x="20" y="{}" class="diagram-title">{}</text>"#,
            y_offset, diagram.name
        ));

        *y_offset += 30.0;

        let mut node_map = HashMap::new();
        collect_nodes(&diagram.root, &mut node_map);

        // Connections
        for conn in &diagram.connections {
            if let (Some(from_node), Some(to_node)) = (
                node_map.get(conn.from.as_str()),
                node_map.get(conn.to.as_str()),
            ) {
                let x1 = from_node.x + (from_node.width / 2.0);
                let y1 = from_node.y + *y_offset + (from_node.height / 2.0);
                let x2 = to_node.x + (to_node.width / 2.0);
                let y2 = to_node.y + *y_offset + (to_node.height / 2.0);
                let mid_y = (y1 + y2) / 2.0;

                let style = match conn.arrow.as_str() {
                    "==" => "stroke-width=\"4\" stroke=\"#333\"",
                    ">>" => "stroke-dasharray=\"5,5\" stroke=\"#666\"",
                    _ => "stroke=\"#999\"",
                };

                let label_svg = if let Some(ref lbl) = conn.label {
                    format!(
                        r#"<text x="{}" y="{}" text-anchor="middle" class="conn-label">{}</text>"#,
                        (x1 + x2) / 2.0,
                        mid_y - 4.0,
                        lbl
                    )
                } else {
                    String::new()
                };

                svg.push_str(&format!(
                    r#"<path d="M {} {} L {} {} L {} {} L {} {}" class="connection" {} />{}"#,
                    x1, y1, x1, mid_y, x2, mid_y, x2, y2, style, label_svg
                ));
            }
        }

        // Nodes
        for node in node_map.values() {
            let node_y = node.y + *y_offset;
            render_node(svg, node, node_y);
        }

        *y_offset += 400.0;
    }
}

/// Render a single node using shape-appropriate SVG and ARIA attributes.
fn render_node(svg: &mut String, node: &Node, y: f32) {
    // --- Milestone 05: Flat Style Buffer ---
    let mut buf = StyleBuffer::new();
    if let Some(fill_val) = node.attributes.get("fill") {
        buf.set_color("fill", fill_val);
    }
    if let Some(stroke_val) = node.attributes.get("stroke") {
        buf.set_color("stroke", stroke_val);
    }
    let fill = buf.get_hex("fill").unwrap_or_else(|| "#ffffff".to_string());
    let stroke = buf.get_hex("stroke").unwrap_or_else(|| "#334155".to_string());

    // --- Milestone 07: ARIA / Interactive DOM Proxy ---
    let tooltip = node.attributes.get("tooltip").cloned();
    let url = node.attributes.get("url").cloned();

    let label = node.label.clone().unwrap_or_else(|| node.id.clone());

    // Wrap in <a> if URL present
    if let Some(ref href) = url {
        svg.push_str(&format!(r#"<a href="{}" target="_blank">"#, href));
    }

    // Emit shape
    let shape_svg = render_shape(node, y, &fill, &stroke);
    svg.push_str(&shape_svg);

    // ARIA title/desc inside the group (use <title> on the path via <g>)
    if let Some(ref tip) = tooltip {
        // Already embedded as <title> inside render_shape wrapper
        let _ = tip; // used below via render_shape_with_aria
    }

    // Label
    svg.push_str(&format!(
        r#"<text x="{}" y="{}" text-anchor="middle" dominant-baseline="middle" class="label">{}</text>"#,
        node.x + node.width / 2.0,
        y + node.height / 2.0,
        label
    ));

    if url.is_some() {
        svg.push_str("</a>");
    }

    // Tooltip title element (SVG native tooltip)
    if let Some(ref tip) = tooltip {
        svg.push_str(&format!(
            r#"<title x="{}" y="{}">{}</title>"#,
            node.x, y, tip
        ));
    }
}

/// Generate the SVG path/shape element for the given node kind.
/// Milestone 03 — Geometric Primitives.
fn render_shape(node: &Node, y: f32, fill: &str, stroke: &str) -> String {
    let x = node.x;
    let w = node.width;
    let h = node.height;
    let style = format!(r#"fill="{}" stroke="{}" stroke-width="2""#, fill, stroke);

    let kind = match node.attributes.get("shape").map(|s| s.as_str()) {
        Some("circle") | Some("ellipse") => ShapeKind::Ellipse,
        Some("diamond") => ShapeKind::Diamond,
        Some("hexagon") => ShapeKind::Hexagon,
        Some("cloud") => ShapeKind::Cloud,
        Some("cylinder") | Some("database") => ShapeKind::Cylinder,
        Some("parallelogram") => ShapeKind::Parallelogram,
        Some("triangle") => ShapeKind::Triangle,
        Some("rounded") => ShapeKind::RoundedRectangle,
        _ => node.kind,
    };

    match kind {
        ShapeKind::Circle | ShapeKind::Ellipse => {
            format!(
                r#"<ellipse cx="{}" cy="{}" rx="{}" ry="{}" class="node" {} />"#,
                x + w / 2.0,
                y + h / 2.0,
                w / 2.0,
                h / 2.0,
                style
            )
        }
        ShapeKind::Diamond => {
            let cx = x + w / 2.0;
            let cy = y + h / 2.0;
            format!(
                r#"<polygon points="{},{} {},{} {},{} {},{}" class="node" {} />"#,
                cx,
                y,
                x + w,
                cy,
                cx,
                y + h,
                x,
                cy,
                style
            )
        }
        ShapeKind::Hexagon => {
            format!(r#"<path d="{}" class="node" {} />"#, hexagon_path(x, y, w, h), style)
        }
        ShapeKind::Cloud => {
            format!(r#"<path d="{}" class="node" {} />"#, cloud_path(x, y, w, h), style)
        }
        ShapeKind::Cylinder | ShapeKind::Database => {
            // Cylinder rendered as rect + top/bottom ellipse arcs
            cylinder_svg(x, y, w, h, fill, stroke)
        }
        ShapeKind::Parallelogram => {
            format!(r#"<path d="{}" class="node" {} />"#, parallelogram_path(x, y, w, h), style)
        }
        ShapeKind::Triangle => {
            format!(
                r#"<polygon points="{},{} {},{} {},{}" class="node" {} />"#,
                x + w / 2.0,
                y,
                x + w,
                y + h,
                x,
                y + h,
                style
            )
        }
        ShapeKind::RoundedRectangle => {
            format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" rx="12" ry="12" class="node" {} />"#,
                x, y, w, h, style
            )
        }
        _ => {
            // Default: plain rect (Box, Rectangle, and all infrastructure shapes)
            format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" rx="5" class="node" {} />"#,
                x, y, w, h, style
            )
        }
    }
}

// ─── Geometric Path Generators ────────────────────────────────────────────────

/// Flat-topped regular hexagon SVG path.
fn hexagon_path(x: f32, y: f32, w: f32, h: f32) -> String {
    let cx = x + w / 2.0;
    let cy = y + h / 2.0;
    let rx = w / 2.0;
    let ry = h / 2.0;
    // 6 vertices at 0°, 60°, 120°, 180°, 240°, 300°
    let pts: Vec<(f32, f32)> = (0..6)
        .map(|i| {
            let angle = PI / 180.0 * (60.0 * i as f32 - 30.0);
            (cx + rx * angle.cos(), cy + ry * angle.sin())
        })
        .collect();
    format!(
        "M {} {} L {} {} L {} {} L {} {} L {} {} L {} {} Z",
        pts[0].0, pts[0].1,
        pts[1].0, pts[1].1,
        pts[2].0, pts[2].1,
        pts[3].0, pts[3].1,
        pts[4].0, pts[4].1,
        pts[5].0, pts[5].1,
    )
}

/// Cloud shape approximated with cubic Bézier arcs.
fn cloud_path(x: f32, y: f32, w: f32, h: f32) -> String {
    let lx = x + w * 0.1;
    let rx = x + w - w * 0.1;
    let cx = x + w / 2.0;
    let ty = y + h * 0.30;
    let cy = (y + y + h) / 2.0;
    let by = y + h - h * 0.30;
    let cx1 = x + w * 0.25;
    let cx2 = x + w - w * 0.25;
    format!(
        "M {lx},{cy} C {lx},{ty} {cx1},{ty} {cx},{ty} C {cx2},{ty} {rx},{ty} {rx},{cy} C {rx},{by} {cx2},{by} {cx},{by} C {cx1},{by} {lx},{by} {lx},{cy} Z",
        lx=lx, rx=rx, cx=cx, ty=ty, cy=cy, by=by, cx1=cx1, cx2=cx2,
    )
}

/// Parallelogram with a 15% horizontal shear.
fn parallelogram_path(x: f32, y: f32, w: f32, h: f32) -> String {
    let skew = w * 0.15;
    format!(
        "M {},{} L {},{} L {},{} L {},{} Z",
        x + skew, y,
        x + w, y,
        x + w - skew, y + h,
        x, y + h,
    )
}

/// Cylinder shape: rect body + top ellipse arc.
fn cylinder_svg(x: f32, y: f32, w: f32, h: f32, fill: &str, stroke: &str) -> String {
    let ry = h * 0.12; // ellipse half-height at top/bottom
    let style = format!(r#"fill="{}" stroke="{}" stroke-width="2""#, fill, stroke);
    format!(
        r#"<rect x="{x}" y="{ty}" width="{w}" height="{bh}" {style} />
           <ellipse cx="{cx}" cy="{ty}" rx="{rx}" ry="{ry}" {style} />
           <ellipse cx="{cx}" cy="{by}" rx="{rx}" ry="{ry}" {style} />"#,
        x = x,
        ty = y + ry,
        w = w,
        bh = h - ry,
        cx = x + w / 2.0,
        rx = w / 2.0,
        ry = ry,
        by = y + h,
        style = style,
    )
}
