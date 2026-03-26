use crate::ast::{Layer, NamedDiagram, Node};
use std::collections::HashMap;

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
            if let (Some(from_node), Some(to_node)) = (node_map.get(conn.from.as_str()), node_map.get(conn.to.as_str())) {
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

                svg.push_str(&format!(
                    r#"<path d="M {} {} L {} {} L {} {} L {} {}" class="connection" {} />"#,
                    x1, y1, x1, mid_y, x2, mid_y, x2, y2, style
                ));
            }
        }

        // Nodes
        for node in node_map.values() {
            svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" rx="5" class="node" />"#,
                node.x, node.y + *y_offset, node.width, node.height
            ));
            
            let label_text = node.label.clone().unwrap_or_else(|| node.id.clone());
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" text-anchor="middle" class="label">{}</text>"#,
                node.x + (node.width / 2.0),
                node.y + *y_offset + (node.height / 2.0) + 5.0,
                label_text
            ));
        }

        *y_offset += 400.0; 
    }
}
