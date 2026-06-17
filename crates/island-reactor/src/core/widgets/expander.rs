use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Expander {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub header: Option<String>,
    pub header_element: Option<Box<Element>>,
    pub child: Box<Element>,
    pub is_expanded: bool,
    pub on_expanding: Option<Callback<bool>>,
}

impl Expander {
    pub fn new(child: impl Into<Element>) -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            header: None,
            header_element: None,
            child: Box::new(child.into()),
            is_expanded: false,
            on_expanding: None,
        }
    }

    pub fn header(mut self, header: impl Into<String>) -> Self {
        self.header = Some(header.into());
        self.header_element = None;
        self
    }

    pub fn header_element(mut self, header: impl Into<Element>) -> Self {
        self.header = None;
        self.header_element = Some(Box::new(header.into()));
        self
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.is_expanded = expanded;
        self
    }

    pub fn on_expanding<F: Fn(bool) + 'static>(mut self, f: F) -> Self {
        self.on_expanding = Some(Callback::new(f));
        self
    }
}

impl Widget for Expander {
    widget_header!(ControlKind::Expander);

    fn bindings(&self) -> PropBindings {
        crate::core::generated_bindings::expander_bindings(self)
    }

    fn children(&self) -> Children<'_> {
        Children::PositionalSingle(&self.child)
    }

    fn header_element(&self) -> Option<&Element> {
        self.header_element.as_deref()
    }
}

pub fn expander(child: impl Into<Element>) -> Expander {
    Expander::new(child)
}
