use crate::ast::Project;

pub fn render_xys(project: &Project, svg: &mut String, y_offset: &mut f32) {
    for xy in &project.xys {
        svg.push_str(&format!(
            r#"<text x="20" y="{}" class="diagram-title">XY Chart: {}</text>"#,
            y_offset, xy.name
        ));

        *y_offset += 30.0;

        let chart_width = 400.0;
        let chart_height = 200.0;
        let origin_x = 50.0;
        let origin_y = *y_offset + chart_height;

        svg.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#333\" stroke-width=\"2\" />",
            origin_x, origin_y, origin_x + chart_width, origin_y
        ));

        svg.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#333\" stroke-width=\"2\" />",
            origin_x, origin_y, origin_x, origin_y - chart_height
        ));

        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"12px\">{}</text>",
            origin_x + (chart_width / 2.0), origin_y + 20.0, xy.x_axis
        ));

        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"12px\" transform=\"rotate(-90 {} {})\" dy=\"-30\">{}</text>",
            origin_x, origin_y - (chart_height / 2.0), origin_x, origin_y - (chart_height / 2.0), xy.y_axis
        ));

        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" font-size=\"12px\" fill=\"#1976d2\">Data Source: {}</text>",
            origin_x + 20.0, origin_y - chart_height + 20.0, xy.data
        ));

        svg.push_str(&format!(
            "<polyline points=\"{} {} {} {} {} {} {} {}\" fill=\"none\" stroke=\"#e91e63\" stroke-width=\"3\" />",
            origin_x, origin_y, origin_x + 100.0, origin_y - 50.0, origin_x + 200.0, origin_y - 120.0, origin_x + 300.0, origin_y - 80.0
        ));

        *y_offset += chart_height + 50.0;
    }
}
