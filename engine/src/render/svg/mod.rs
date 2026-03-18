use crate::ast::Project;

mod diagram;
mod packet;
mod state;
mod xy;

pub fn render_svg(project: &Project) -> String {
    let mut svg = String::from(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1000 2000">"#);
    
    svg.push_str("<style>
        .node { fill: #fff; stroke: #333; stroke-width: 2; }
        .connection { stroke: #999; stroke-width: 2; fill: none; }
        .label { font-family: sans-serif; font-size: 12px; fill: #333; }
        .diagram-title { font-family: sans-serif; font-size: 18px; font-weight: bold; fill: #000; }
    </style>");

    let mut y_offset = 50.0;

    diagram::render_diagrams(&project.diagrams, &mut svg, &mut y_offset);
    packet::render_packets(project, &mut svg, &mut y_offset);
    state::render_states(project, &mut svg, &mut y_offset);
    xy::render_xys(project, &mut svg, &mut y_offset);

    svg.push_str("</svg>");
    svg
}
