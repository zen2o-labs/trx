use crate::ast::Project;

pub fn render_states(_project: &Project, _svg: &mut String, _y_offset: &mut f32) {
    /*
    for state in &project.states {
        svg.push_str(&format!(
            r#"<text x="20" y="{}" class="diagram-title">State Machine: {}</text>"#,
            y_offset, state.name
        ));

        *y_offset += 30.0;

        let mut x_offset = 50.0;
        let mut state_positions = std::collections::HashMap::new();

        let unique_states: std::collections::HashSet<_> = state.transitions.iter()
            .flat_map(|t| vec![&t.from, &t.to])
            .collect();

        for s in unique_states {
            state_positions.insert(s, (x_offset, y_offset));
            svg.push_str(&format!(
                "<circle cx=\"{}\" cy=\"{}\" r=\"30\" class=\"node\" fill=\"#fafafa\" />",
                x_offset, *y_offset
            ));
            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" class=\"label\" dy=\".3em\">{}</text>",
                x_offset, *y_offset, s
            ));
            x_offset += 150.0;
        }

        for t in &state.transitions {
            if let (Some(&(x1, y1)), Some(&(x2, y2))) = (state_positions.get(&t.from), state_positions.get(&t.to)) {
                svg.push_str(&format!(
                    "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" class=\"connection\" stroke-dasharray=\"5,5\" />",
                    x1 + 30.0, y1, x2 - 30.0, y2
                ));

                if let Some(trigger) = &t.trigger {
                    svg.push_str(&format!(
                        "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"10px\" fill=\"#d32f2f\">{}</text>",
                        (x1 + x2) / 2.0, y1 - 10.0, trigger
                    ));
                }
            }
        }

        *y_offset += 100.0;
    }
    */
}
