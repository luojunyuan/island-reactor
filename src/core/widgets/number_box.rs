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
        let mut out = vec![
            Binding::Prop(Prop::Value, PropValue::F64(self.value)),
            Binding::Prop(Prop::Minimum, PropValue::F64(self.minimum)),
            Binding::Prop(Prop::Maximum, PropValue::F64(self.maximum)),
            Binding::Prop(Prop::IsEnabled, PropValue::Bool(self.is_enabled)),
            Binding::Event(
                Event::ValueChanged,
                self.on_value_changed
                    .as_ref()
                    .map(|cb| EventHandler::F64(cb.clone())),
            ),
        ];
        if let Some(header) = &self.header {
            out.push(Binding::Prop(Prop::Header, PropValue::Str(header.clone())));
        }
        out
    }
}

pub fn number_box(value: f64) -> NumberBox {
    NumberBox::new(value)
}
