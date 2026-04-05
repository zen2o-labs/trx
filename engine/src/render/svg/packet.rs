use crate::ast::Project;

pub fn render_packets(project: &Project, svg: &mut String, y_offset: &mut f32) {
    for packet in &project.packets {
        svg.push_str(&format!(
            r#"<text x="20" y="{}" class="diagram-title">Packet: {}</text>"#,
            y_offset, packet.name
        ));

        *y_offset += 30.0;
        let mut x_offset = 20.0;
        let packet_height = 50.0;

        for field in &packet.fields {
            let field_width = 120.0; // In a full impl this would be derived from field.range

            svg.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"0\" class=\"node\" fill=\"#e3f2fd\" />",
                x_offset, y_offset, field_width, packet_height
            ));

            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" class=\"label\">{}</text>",
                x_offset + (field_width / 2.0),
                *y_offset + (packet_height / 2.0) - 5.0,
                field.name
            ));

            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"10px\" fill=\"#666\">{}</text>",
                x_offset + (field_width / 2.0),
                *y_offset + (packet_height / 2.0) + 10.0,
                field.range
            ));

            x_offset += field_width;
        }

        *y_offset += 100.0;
    }
}
