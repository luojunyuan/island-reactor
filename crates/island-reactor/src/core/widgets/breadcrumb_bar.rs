use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct BreadcrumbBar {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub items: Vec<String>,
    pub on_item_clicked: Option<Callback<i32>>,
}

impl BreadcrumbBar {
    pub fn new<I, S>(items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            items: items.into_iter().map(Into::into).collect(),
            on_item_clicked: None,
        }
    }

    pub fn on_item_clicked<F: Fn(i32) + 'static>(mut self, f: F) -> Self {
        self.on_item_clicked = Some(Callback::new(f));
        self
    }
}

impl Widget for BreadcrumbBar {
    widget_header!(ControlKind::BreadcrumbBar);

    fn bindings(&self) -> PropBindings {
        let mut out = crate::core::generated_bindings::breadcrumb_bar_bindings(self);
        out.push(Binding::Prop(
            Prop::Items,
            PropValue::StrList(self.items.clone()),
        ));
        out
    }
}

pub fn breadcrumb_bar<I, S>(items: I) -> BreadcrumbBar
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    BreadcrumbBar::new(items)
}
