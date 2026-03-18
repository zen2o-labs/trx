use crate::ast::NamedDiagram;

pub fn render_diagrams(diagrams: &[NamedDiagram], svg: &mut String, y_offset: &mut f32) {
    for diagram in diagrams {
        svg.push_str(&format!(
            r#"<text x="20" y="{}" class="diagram-title">{}</text>"#,
            y_offset, diagram.name
        ));
        
        *y_offset += 30.0;
        
        // Connections
        for i in 0..diagram.connections_from.len() {
            let from_idx = diagram.connections_from[i] as usize;
            let to_idx = diagram.connections_to[i] as usize;
            
            let x1 = diagram.node_x[from_idx] + 60.0;
            let y1 = diagram.node_y[from_idx] + *y_offset as f32 + 30.0;
            let x2 = diagram.node_x[to_idx] + 60.0;
            let y2 = diagram.node_y[to_idx] + *y_offset as f32 + 30.0;
            
            let mid_y = (y1 + y2) / 2.0;
            
            svg.push_str(&format!(
                r#"<path d="M {} {} L {} {} L {} {} L {} {}" class="connection" />"#,
                x1, y1, x1, mid_y, x2, mid_y, x2, y2
            ));
        }

        // Nodes
        for i in 0..diagram.node_ids.len() {
            svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" rx="5" class="node" />"#,
                diagram.node_x[i], diagram.node_y[i] + *y_offset as f32, diagram.node_width[i], diagram.node_height[i]
            ));
            
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" text-anchor="middle" class="label">{}</text>"#,
                diagram.node_x[i] + (diagram.node_width[i] / 2.0),
                diagram.node_y[i] + *y_offset as f32 + (diagram.node_height[i] / 2.0) + 5.0,
                diagram.node_labels[i]
            ));
        }

        *y_offset += 400.0; 
    }
}
