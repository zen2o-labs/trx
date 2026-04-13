use crate::ast::Project;

// Constants for XY chart rendering
const CHART_WIDTH: f32 = 400.0;
const CHART_HEIGHT: f32 = 200.0;
const ORIGIN_X_OFFSET: f32 = 50.0;
const TITLE_PADDING: f32 = 30.0;
const BOTTOM_PADDING: f32 = 50.0;

pub fn render_xys(project: &Project, svg: &mut String, y_offset: &mut f32) {
    for xy in &project.xys {
        // Render Chart Title
        svg.push_str(&format!(
            r#"<text x="20" y="{}" class="diagram-title">XY Chart: {}</text>"#,
            y_offset, xy.name
        ));

        *y_offset += TITLE_PADDING;

        let origin_x = ORIGIN_X_OFFSET;
        let origin_y = *y_offset + CHART_HEIGHT;

        // Draw Axes
        // Draw X-axis
        svg.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#333\" stroke-width=\"2\" />",
            origin_x,
            origin_y,
            origin_x + CHART_WIDTH,
            origin_y
        ));
        // Draw Y-axis
        svg.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#333\" stroke-width=\"2\" />",
            origin_x,
            origin_y,
            origin_x,
            origin_y - CHART_HEIGHT
        ));

        // Axis Labels
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"12px\">{}</text>",
            origin_x + (CHART_WIDTH / 2.0),
            origin_y + 20.0,
            xy.x_axis
        ));

        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"12px\" transform=\"rotate(-90 {} {})\" dy=\"-30\">{}</text>",
            origin_x, origin_y - (CHART_HEIGHT / 2.0), origin_x, origin_y - (CHART_HEIGHT / 2.0), xy.y_axis
        ));

        // Data Points Parsing
        // We parse the string data format "x1,y1 x2,y2 ..." into SVG polyline points.
        let mut polyline_points = String::new();
        let raw_data = xy.data.trim();

        // Find min/max for scaling
        let mut points = Vec::new();
        for pair in raw_data.split_whitespace() {
            if let Some((x_str, y_str)) = pair.split_once(',') {
                if let (Ok(x_val), Ok(y_val)) = (x_str.parse::<f32>(), y_str.parse::<f32>()) {
                    points.push((x_val, y_val));
                }
            }
        }

        if !points.is_empty() {
            let max_x = points.iter().map(|p| p.0).fold(0.0, f32::max).max(1.0);
            let max_y = points.iter().map(|p| p.1).fold(0.0, f32::max).max(1.0);

            for (x_val, y_val) in points {
                let px = origin_x + (x_val / max_x) * CHART_WIDTH;
                let py = origin_y - (y_val / max_y) * CHART_HEIGHT;
                polyline_points.push_str(&format!("{:.2},{:.2} ", px, py));
            }

            svg.push_str(&format!(
                "<polyline points=\"{}\" fill=\"none\" stroke=\"#e91e63\" stroke-width=\"3\" />",
                polyline_points.trim()
            ));
        }

        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" font-size=\"12px\" fill=\"#1976d2\">Data Source: {}</text>",
            origin_x + 20.0,
            origin_y - CHART_HEIGHT + 20.0,
            xy.data
        ));

        *y_offset += CHART_HEIGHT + BOTTOM_PADDING;
    }
}
