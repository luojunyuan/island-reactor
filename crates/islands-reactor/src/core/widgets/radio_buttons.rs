use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct RadioButtons {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub items: Vec<String>,
    pub selected_index: i32,
    pub header: Option<String>,
    pub max_columns: i32,
    pub on_selection_changed: Option<Callback<i32>>,
}

impl RadioButtons {
    pub fn new<I, S>(items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            items: items.into_iter().map(Into::into).collect(),
            selected_index: -1,
            header: None,
            max_columns: 1,
            on_selection_changed: None,
        }
    }

    pub fn selected_index(mut self, selected_index: i32) -> Self {
        self.selected_index = selected_index;
        self
    }

    pub fn header(mut self, header: impl Into<String>) -> Self {
        self.header = Some(header.into());
        self
    }

    pub fn max_columns(mut self, max_columns: i32) -> Self {
        self.max_columns = max_columns;
        self
    }

    pub fn on_selection_changed<F: Fn(i32) + 'static>(mut self, f: F) -> Self {
        self.on_selection_changed = Some(Callback::new(f));
        self
    }
}

impl Widget for RadioButtons {
    widget_header!(ControlKind::RadioButtons);

    fn bindings(&self) -> PropBindings {
        let mut out = crate::core::generated_bindings::radio_buttons_bindings(self);
        out.push(Binding::Prop(
            Prop::Items,
            PropValue::StrList(self.items.clone()),
        ));
        out
    }
}

pub fn radio_buttons<I, S>(items: I) -> RadioButtons
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    RadioButtons::new(items)
}
