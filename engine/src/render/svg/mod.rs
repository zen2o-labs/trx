use crate::ast::Project;

mod diagram;
mod packet;
mod sqltable;
mod state;
mod xy;

pub fn render_svg(project: &Project) -> String {
    let mut svg = String::from(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1200 3000">"#);

    svg.push_str("<style>
        .node { fill: #ffffff; stroke: #334155; stroke-width: 2; }
        .connection { stroke: #64748b; stroke-width: 2; fill: none; }
        .label { font-family: Inter, sans-serif; font-size: 13px; fill: #1e293b; }
        .conn-label { font-family: Inter, sans-serif; font-size: 11px; fill: #64748b; }
        .diagram-title { font-family: Inter, sans-serif; font-size: 20px; font-weight: 700; fill: #0f172a; }
    </style>");

    let mut y_offset = 50.0;

    diagram::render_diagrams(&project.diagrams, &mut svg, &mut y_offset);
    packet::render_packets(project, &mut svg, &mut y_offset);
    sqltable::render_sqltables(project, &mut svg, &mut y_offset);
    state::render_states(project, &mut svg, &mut y_offset);
    xy::render_xys(project, &mut svg, &mut y_offset);

    svg.push_str("</svg>");
    svg
}
