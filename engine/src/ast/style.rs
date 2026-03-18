use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value as JsonValue;
use crate::ast::element::ShapeKind;

#[derive(Clone)]
pub struct NodeView<'a> {
    pub id: &'a str,
    pub kind: &'a ShapeKind,
    pub tags: &'a [String],
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Theme {
    #[serde(default)]
    pub variables: HashMap<String, String>,
    #[serde(default)]
    pub rules: Vec<StyleRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleRule {
    pub selectors: Vec<StyleSelector>,
    #[serde(default)]
    pub properties: StyleProperties,
    pub pseudo: Option<String>,
    #[serde(default)]
    pub important: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StyleProperties {
    pub fill: Option<String>,
    pub stroke: Option<String>,
    pub stroke_width: Option<f64>,
    pub opacity: Option<f64>,
    pub shadow: Option<bool>,
    pub corner_radius: Option<f64>,
    pub text_color: Option<String>,
    pub font_size: Option<f64>,
    pub font_weight: Option<String>,
    pub font_style: Option<String>,
    pub dash_pattern: Option<String>,
    pub start_arrow: Option<String>,
    pub end_arrow: Option<String>,
    #[serde(default)]
    pub custom: HashMap<String, JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum StyleSelector {
    Id { value: String },
    Class { value: String },
    Type { shape: ShapeKind },
    Connection,
    Descendant { ancestor: Box<StyleSelector>, descendant: Box<StyleSelector> },
    Child { parent: Box<StyleSelector>, child: Box<StyleSelector> },
}

pub struct StyleResolver<'a> {
    theme: &'a Theme,
}

impl<'a> StyleResolver<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        Self { theme }
    }

    pub fn resolve(
        &self,
        node: &NodeView,
        parent_chain: &[&NodeView],
        pseudo: Option<&str>,
    ) -> StyleProperties {
        let mut matched = Vec::new();
        let context = ResolutionContext { node, parent_chain, pseudo };

        for rule in &self.theme.rules {
            if rule.pseudo.as_deref() != pseudo { continue; }
            if rule.selectors.iter().any(|s| self.matches(s, &context)) {
                let specificity = self.calculate_specificity(&rule.selectors);
                matched.push((specificity, rule.important, &rule.properties));
            }
        }

        matched.sort_by_key(|(spec, important, _)| (*important, *spec));

        let mut final_props = StyleProperties::default();
        for (_, _, props) in matched {
            final_props.merge(props.clone(), self.theme);
        }
        final_props
    }

    fn matches(&self, selector: &StyleSelector, ctx: &ResolutionContext) -> bool {
        match selector {
            StyleSelector::Id { value } => ctx.node.id == value,

            StyleSelector::Class { value } => ctx.node.tags.contains(value),

            StyleSelector::Type { shape } => ctx.node.kind == shape,

            StyleSelector::Connection => false,

            StyleSelector::Descendant { ancestor, descendant } => {
                if !self.matches(descendant, ctx) { return false; }
                ctx.parent_chain.iter().enumerate().any(|(i, parent)| {
                    let parent_ctx = ResolutionContext {
                        node: parent,
                        parent_chain: &ctx.parent_chain[i+1..],
                        pseudo: ctx.pseudo,
                    };
                    self.matches(ancestor, &parent_ctx)
                })
            }
            StyleSelector::Child { parent: p_sel, child: c_sel } => {
                if !self.matches(c_sel, ctx) { return false; }
                ctx.parent_chain.first().map_or(false, |p_node| {
                    let parent_ctx = ResolutionContext {
                        node: p_node,
                        parent_chain: &ctx.parent_chain[1..],
                        pseudo: ctx.pseudo,
                    };
                    self.matches(p_sel, &parent_ctx)
                })
            }
        }
    }

    fn calculate_specificity(&self, selectors: &[StyleSelector]) -> u32 {
        let mut score = 0;
        for s in selectors {
            score += match s {
                StyleSelector::Id { .. } => 100,
                StyleSelector::Class { .. } => 10,
                _ => 1,
            };
        }
        score
    }
}

impl StyleProperties {
    pub fn merge(&mut self, other: StyleProperties, theme: &Theme) {
        if let Some(v) = other.fill { self.fill = Some(resolve_var(v, theme)); }
        if let Some(v) = other.stroke { self.stroke = Some(resolve_var(v, theme)); }
        if let Some(v) = other.stroke_width { self.stroke_width = Some(v); }
        if let Some(v) = other.opacity { self.opacity = Some(v); }
        if let Some(v) = other.font_size { self.font_size = Some(v); }
    }
}

fn resolve_var(s: String, theme: &Theme) -> String {
    if s.starts_with("var(") && s.ends_with(')') {
        let key = s[4..s.len()-1].trim();
        theme.variables.get(key).cloned().unwrap_or(s)
    } else { s }
}

pub struct ResolutionContext<'a> {
    pub node: &'a NodeView<'a>,
    pub parent_chain: &'a [&'a NodeView<'a>],
    pub pseudo: Option<&'a str>,
}
