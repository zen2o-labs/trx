use crate::ast::Project;

mod diagram;
mod packet;
mod sqltable;
mod state;
mod xy;

const PAN_ZOOM_SCRIPT: &str = r#"
<script type="text/javascript">
<![CDATA[
(function() {
    const svg = document.documentElement;
    let isDragging = false;
    let startX = 0;
    let startY = 0;
    let viewBoxX = 0;
    let viewBoxY = 0;
    let viewBoxWidth = 0;
    let viewBoxHeight = 0;

    function parseViewBox() {
        const vb = svg.getAttribute("viewBox");
        if (vb) {
            const parts = vb.split(" ").map(Number);
            if (parts.length === 4) {
                viewBoxX = parts[0];
                viewBoxY = parts[1];
                viewBoxWidth = parts[2];
                viewBoxHeight = parts[3];
            }
        }
    }

    parseViewBox();

    svg.addEventListener("mousedown", (e) => {
        isDragging = true;
        startX = e.clientX;
        startY = e.clientY;
        svg.style.cursor = "grabbing";
    });

    svg.addEventListener("mousemove", (e) => {
        if (!isDragging) return;
        const dx = e.clientX - startX;
        const dy = e.clientY - startY;

        const clientRect = svg.getBoundingClientRect();
        const scaleX = viewBoxWidth / clientRect.width;
        const scaleY = viewBoxHeight / clientRect.height;

        viewBoxX -= dx * scaleX;
        viewBoxY -= dy * scaleY;

        svg.setAttribute("viewBox", `${viewBoxX} ${viewBoxY} ${viewBoxWidth} ${viewBoxHeight}`);

        startX = e.clientX;
        startY = e.clientY;
    });

    const stopDragging = () => {
        isDragging = false;
        svg.style.cursor = "grab";
    };

    svg.addEventListener("mouseup", stopDragging);
    svg.addEventListener("mouseleave", stopDragging);

    svg.addEventListener("wheel", (e) => {
        e.preventDefault();
        parseViewBox();
        const clientRect = svg.getBoundingClientRect();
        const mouseX = e.clientX - clientRect.left;
        const mouseY = e.clientY - clientRect.top;

        const svgX = viewBoxX + (mouseX / clientRect.width) * viewBoxWidth;
        const svgY = viewBoxY + (mouseY / clientRect.height) * viewBoxHeight;

        const zoomFactor = 1.0 + (e.deltaY * 0.001);

        viewBoxWidth *= zoomFactor;
        viewBoxHeight *= zoomFactor;

        viewBoxX = svgX - (mouseX / clientRect.width) * viewBoxWidth;
        viewBoxY = svgY - (mouseY / clientRect.height) * viewBoxHeight;

        svg.setAttribute("viewBox", `${viewBoxX} ${viewBoxY} ${viewBoxWidth} ${viewBoxHeight}`);
    }, { passive: false });
})();
]]>
</script>
"#;

fn round_layer(layer: &mut crate::ast::Layer) {
    for node in &mut layer.nodes {
        node.x = (node.x * 1000.0).round() / 1000.0;
        node.y = (node.y * 1000.0).round() / 1000.0;
        node.width = (node.width * 1000.0).round() / 1000.0;
        node.height = (node.height * 1000.0).round() / 1000.0;
    }
    for l in &mut layer.layers {
        round_layer(l);
    }
}

pub fn render_svg(project: &Project) -> String {
    let mut rounded_project = project.clone();
    for diag in &mut rounded_project.diagrams {
        round_layer(&mut diag.root);
    }

    // Default dimensions, will be overwritten by dynamic calculation if possible
    let mut content_svg = String::new();
    let mut y_offset = 50.0;

    diagram::render_diagrams(
        &rounded_project.diagrams,
        &rounded_project.classes,
        &mut content_svg,
        &mut y_offset,
    );
    packet::render_packets(&rounded_project, &mut content_svg, &mut y_offset);
    sqltable::render_sqltables(&rounded_project, &mut content_svg, &mut y_offset);
    state::render_states(&rounded_project, &mut content_svg, &mut y_offset);
    xy::render_xys(&rounded_project, &mut content_svg, &mut y_offset);

    // Dynamic Bounding Box Calculation
    // We add some bottom padding to the calculated y_offset to ensure all elements are visible.
    let canvas_height = y_offset + 100.0;
    let canvas_width = 1200.0; // Standard default width

    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="100%" height="100%" style="cursor: grab;">"#,
        canvas_width, canvas_height
    );

    svg.push_str("<style>
        .node { fill: #ffffff; stroke: #334155; stroke-width: 2; }
        .connection { stroke: #64748b; stroke-width: 2; fill: none; }
        .label { font-family: Inter, sans-serif; font-size: 13px; fill: #1e293b; }
        .conn-label { font-family: Inter, sans-serif; font-size: 11px; fill: #64748b; }
        .diagram-title { font-family: Inter, sans-serif; font-size: 20px; font-weight: 700; fill: #0f172a; }
    </style>");

    svg.push_str(&content_svg);
    svg.push_str(PAN_ZOOM_SCRIPT);
    svg.push_str("</svg>");
    svg
}
