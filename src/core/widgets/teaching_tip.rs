use super::*;

pub use muxc_bindings::TeachingTipPlacementMode;

#[derive(Clone, Debug, PartialEq)]
pub struct TeachingTip {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub title: String,
    pub subtitle: String,
    pub is_open: bool,
    pub is_light_dismiss_enabled: bool,
    pub preferred_placement: TeachingTipPlacementMode,
    pub action_button_text: Option<String>,
    pub close_button_text: Option<String>,
    pub on_closed: Option<Callback<()>>,
    pub on_action_button_click: Option<Callback<()>>,
}

impl TeachingTip {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            title: title.into(),
            subtitle: String::new(),
            is_open: false,
            is_light_dismiss_enabled: false,
            preferred_placement: TeachingTipPlacementMode::Auto,
            action_button_text: None,
            close_button_text: None,
            on_closed: None,
            on_action_button_click: None,
        }
    }

    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = subtitle.into();
        self
    }

    pub fn is_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn light_dismiss(mut self) -> Self {
        self.is_light_dismiss_enabled = true;
        self
    }

    pub fn preferred_placement(mut self, placement: TeachingTipPlacementMode) -> Self {
        self.preferred_placement = placement;
        self
    }

    pub fn action_button(mut self, text: impl Into<String>) -> Self {
        self.action_button_text = Some(text.into());
        self
    }

    pub fn close_button(mut self, text: impl Into<String>) -> Self {
        self.close_button_text = Some(text.into());
        self
    }

    pub fn on_closed<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_closed = Some(Callback::new(move |()| f()));
        self
    }

    pub fn on_action_button_click<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_action_button_click = Some(Callback::new(move |()| f()));
        self
    }
}

impl Widget for TeachingTip {
    widget_header!(ControlKind::TeachingTip);

    fn bindings(&self) -> PropBindings {
        let mut out = vec![
            Binding::Prop(Prop::Title, PropValue::Str(self.title.clone())),
            Binding::Prop(Prop::Subtitle, PropValue::Str(self.subtitle.clone())),
            Binding::Prop(Prop::IsOpen, PropValue::Bool(self.is_open)),
            Binding::Prop(
                Prop::IsLightDismissEnabled,
                PropValue::Bool(self.is_light_dismiss_enabled),
            ),
            Binding::Prop(
                Prop::PreferredPlacement,
                PropValue::I32(self.preferred_placement.0),
            ),
            Binding::Event(
                Event::Closed,
                self.on_closed
                    .as_ref()
                    .map(|cb| EventHandler::Unit(cb.clone())),
            ),
            Binding::Event(
                Event::ActionButtonClick,
                self.on_action_button_click
                    .as_ref()
                    .map(|cb| EventHandler::Unit(cb.clone())),
            ),
        ];
        if let Some(text) = &self.action_button_text {
            out.push(Binding::Prop(
                Prop::ActionButtonText,
                PropValue::Str(text.clone()),
            ));
        }
        if let Some(text) = &self.close_button_text {
            out.push(Binding::Prop(
                Prop::CloseButtonText,
                PropValue::Str(text.clone()),
            ));
        }
        out
    }
}

pub fn teaching_tip(title: impl Into<String>) -> TeachingTip {
    TeachingTip::new(title)
}
