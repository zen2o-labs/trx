use crate::ast::Project;

fn parse_bit_width(range: &str) -> f32 {
    let range = range.trim();
    if let Some(idx) = range.find("..") {
        if let (Ok(start), Ok(end)) = (
            range[..idx].trim().parse::<u32>(),
            range[idx + 2..].trim().parse::<u32>(),
        ) {
            return (end.saturating_sub(start) + 1) as f32;
        }
    } else if let Some(idx) = range.find('-') {
        if let (Ok(start), Ok(end)) = (
            range[..idx].trim().parse::<u32>(),
            range[idx + 1..].trim().parse::<u32>(),
        ) {
            return (end.saturating_sub(start) + 1) as f32;
        }
    } else if let Ok(_) = range.parse::<u32>() {
        return 1.0;
    }
    16.0
}

pub fn render_packets(project: &Project, svg: &mut String, y_offset: &mut f32) {
    for packet in &project.packets {
        svg.push_str(&format!(
            r#"<text x="20" y="{}" class="diagram-title">Packet: {}</text>"#,
            y_offset, packet.name
        ));

        *y_offset += 30.0;
        let mut x_offset = 20.0;
        let packet_height = 50.0;
        let group_width = 800.0; // Fixed total width for visualization

        let total_bits: f32 = packet
            .fields
            .iter()
            .map(|f| parse_bit_width(&f.range))
            .sum();

        if total_bits == 0.0 {
            svg.push_str(&format!(
                "<text x=\"20\" y=\"{}\" fill=\"#ef4444\" font-size=\"12px\">[Empty/Invalid Bit Definition]</text>",
                *y_offset + 20.0
            ));
            *y_offset += 50.0;
            continue;
        }

        for field in &packet.fields {
            let bits = parse_bit_width(&field.range);
            let field_width = (bits / total_bits) * group_width;

            svg.push_str(&format!(
                "<rect x=\"{:.3}\" y=\"{:.3}\" width=\"{:.3}\" height=\"{:.3}\" rx=\"0\" class=\"node\" fill=\"#e3f2fd\" stroke=\"#334155\" stroke-width=\"1\" />",
                x_offset, y_offset, field_width, packet_height
            ));

            svg.push_str(&format!(
                "<text x=\"{:.3}\" y=\"{:.3}\" text-anchor=\"middle\" class=\"label\">{}</text>",
                x_offset + (field_width / 2.0),
                *y_offset + (packet_height / 2.0) - 5.0,
                field.name
            ));

            svg.push_str(&format!(
                "<text x=\"{:.3}\" y=\"{:.3}\" text-anchor=\"middle\" font-size=\"10px\" fill=\"#666\">{}</text>",
                x_offset + (field_width / 2.0),
                *y_offset + (packet_height / 2.0) + 10.0,
                field.range
            ));

            x_offset += field_width;
        }

        *y_offset += 100.0;
    }
}
