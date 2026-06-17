use super::*;

pub use crate::bindings_muxc::InfoBarSeverity;

#[derive(Clone, Debug, PartialEq)]
pub struct InfoBar {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub title: String,
    pub message: String,
    pub severity: InfoBarSeverity,
    pub is_open: bool,
    pub is_closable: bool,
    pub on_closed: Option<Callback<()>>,
}

impl Default for InfoBar {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            title: String::new(),
            message: String::new(),
            severity: InfoBarSeverity::Informational,
            is_open: false,
            is_closable: true,
            on_closed: None,
        }
    }
}

impl InfoBar {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            is_open: true,
            ..Default::default()
        }
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    pub fn severity(mut self, severity: InfoBarSeverity) -> Self {
        self.severity = severity;
        self
    }

    pub fn informational(self) -> Self {
        self.severity(InfoBarSeverity::Informational)
    }

    pub fn success(self) -> Self {
        self.severity(InfoBarSeverity::Success)
    }

    pub fn warning(self) -> Self {
        self.severity(InfoBarSeverity::Warning)
    }

    pub fn error(self) -> Self {
        self.severity(InfoBarSeverity::Error)
    }

    pub fn is_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn is_closable(mut self, is_closable: bool) -> Self {
        self.is_closable = is_closable;
        self
    }

    pub fn on_closed<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_closed = Some(Callback::new(move |()| f()));
        self
    }
}

impl Widget for InfoBar {
    widget_header!(ControlKind::InfoBar);

    fn bindings(&self) -> PropBindings {
        crate::core::generated_bindings::info_bar_bindings(self)
    }
}

pub fn info_bar(title: impl Into<String>) -> InfoBar {
    InfoBar::new(title)
}
