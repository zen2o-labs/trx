/// Milestone 04 — Schema Visualization
/// Renders `sqltable` declarations as SVG table diagrams with PK/FK indicators.
use crate::ast::{Project, SqlTableDeclaration};

pub fn render_sqltables(project: &Project, svg: &mut String, y_offset: &mut f32) {
    for table in &project.sqltables {
        render_table(table, svg, *y_offset);
        *y_offset += table_height(table) + 40.0;
    }
}

const COL_W: f32 = 240.0;
const ROW_H: f32 = 28.0;
const HEADER_H: f32 = 36.0;
const X_START: f32 = 20.0;

fn table_height(table: &SqlTableDeclaration) -> f32 {
    HEADER_H + table.fields.len() as f32 * ROW_H + 4.0
}

fn render_table(table: &SqlTableDeclaration, svg: &mut String, y: f32) {
    let total_h = table_height(table);
    let x = X_START;
    let w = COL_W;

    // Table border
    svg.push_str(&format!(
        "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"6\" fill=\"#1e293b\" stroke=\"#334155\" stroke-width=\"2\"/>",
        x, y, w, total_h
    ));

    // Header background
    svg.push_str(&format!(
        "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"6\" fill=\"#334155\"/>",
        x, y, w, HEADER_H
    ));

    // Table name
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-weight=\"bold\" fill=\"#f8fafc\" font-size=\"14\" font-family=\"monospace\">{}</text>",
        x + w / 2.0,
        y + HEADER_H / 2.0 + 5.0,
        table.name
    ));

    // Field rows
    for (i, field) in table.fields.iter().enumerate() {
        let row_y = y + HEADER_H + i as f32 * ROW_H;

        // Alternating row background
        if i % 2 == 0 {
            svg.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#0f172a\" opacity=\"0.4\"/>",
                x, row_y, w, ROW_H
            ));
        }

        let mut badge_x = x + 6.0;

        // PK badge
        if field.is_pk {
            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" fill=\"#fbbf24\" font-size=\"10\" font-weight=\"bold\" font-family=\"monospace\">PK</text>",
                badge_x,
                row_y + ROW_H / 2.0 + 4.0,
            ));
            badge_x += 20.0;
        }

        // FK badge
        if field.is_fk {
            let fk_label = field.fk_ref.as_deref().unwrap_or("FK");
            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" fill=\"#60a5fa\" font-size=\"9\" font-family=\"monospace\">-&gt;{}</text>",
                badge_x,
                row_y + ROW_H / 2.0 + 4.0,
                fk_label,
            ));
            badge_x += 30.0;
        }

        // Field name
        let text_x = if field.is_pk || field.is_fk { badge_x } else { x + 10.0 };
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" fill=\"#e2e8f0\" font-size=\"12\" font-family=\"monospace\">{}</text>",
            text_x,
            row_y + ROW_H / 2.0 + 4.0,
            field.name
        ));

        // Field type (right-aligned)
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" text-anchor=\"end\" fill=\"#94a3b8\" font-size=\"11\" font-family=\"monospace\">{}</text>",
            x + w - 8.0,
            row_y + ROW_H / 2.0 + 4.0,
            field.field_type
        ));

        // Row separator
        svg.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#334155\" stroke-width=\"1\"/>",
            x,
            row_y + ROW_H,
            x + w,
            row_y + ROW_H
        ));
    }
}
