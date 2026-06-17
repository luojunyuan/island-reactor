use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct NumberBox {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub value: f64,
    pub minimum: f64,
    pub maximum: f64,
    pub header: Option<String>,
    pub is_enabled: bool,
    pub on_value_changed: Option<Callback<f64>>,
}

impl Default for NumberBox {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            value: 0.0,
            minimum: f64::MIN,
            maximum: f64::MAX,
            header: None,
            is_enabled: true,
            on_value_changed: None,
        }
    }
}

impl NumberBox {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }

    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.minimum = min;
        self.maximum = max;
        self
    }

    pub fn header(mut self, header: impl Into<String>) -> Self {
        self.header = Some(header.into());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }

    pub fn on_value_changed<F: Fn(f64) + 'static>(mut self, f: F) -> Self {
        self.on_value_changed = Some(Callback::new(f));
        self
    }
}

impl Widget for NumberBox {
    widget_header!(ControlKind::NumberBox);

    fn bindings(&self) -> PropBindings {
        crate::core::generated_bindings::number_box_bindings(self)
    }
}

pub fn number_box(value: f64) -> NumberBox {
    NumberBox::new(value)
}
