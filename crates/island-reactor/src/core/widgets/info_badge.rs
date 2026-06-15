use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct InfoBadge {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub value: Option<i32>,
}

impl InfoBadge {
    pub fn new() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            value: None,
        }
    }

    pub fn dot() -> Self {
        Self::new()
    }

    pub fn numeric(value: i32) -> Self {
        Self::new().value(value)
    }

    pub fn value(mut self, value: i32) -> Self {
        self.value = Some(value);
        self
    }
}

impl Default for InfoBadge {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for InfoBadge {
    widget_header!(ControlKind::InfoBadge);

    fn bindings(&self) -> PropBindings {
        self.value
            .map(|value| vec![Binding::Prop(Prop::Value, PropValue::I32(value))])
            .unwrap_or_default()
    }
}

pub fn info_badge() -> InfoBadge {
    InfoBadge::new()
}
