use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct TabItem {
    pub key: Option<String>,
    pub header: String,
    pub content: Element,
    pub is_closable: Option<bool>,
}

impl TabItem {
    pub fn new(header: impl Into<String>, content: impl Into<Element>) -> Self {
        Self {
            key: None,
            header: header.into(),
            content: content.into(),
            is_closable: None,
        }
    }

    pub fn with_key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn closable(mut self, closable: bool) -> Self {
        self.is_closable = Some(closable);
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TabView {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub tabs: Vec<TabItem>,
    pub selected_index: i32,
    pub can_reorder_tabs: bool,
    pub is_add_tab_button_visible: bool,
    pub on_selection_changed: Option<Callback<i32>>,
    pub on_close_requested: Option<Callback<String>>,
    pub on_add_tab_button_click: Option<Callback<()>>,
}

impl TabView {
    pub fn new<I: IntoIterator<Item = TabItem>>(tabs: I) -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            tabs: tabs.into_iter().collect(),
            selected_index: 0,
            can_reorder_tabs: false,
            is_add_tab_button_visible: false,
            on_selection_changed: None,
            on_close_requested: None,
            on_add_tab_button_click: None,
        }
    }

    pub fn selected_index(mut self, selected_index: i32) -> Self {
        self.selected_index = selected_index;
        self
    }

    pub fn can_reorder_tabs(mut self, can_reorder_tabs: bool) -> Self {
        self.can_reorder_tabs = can_reorder_tabs;
        self
    }

    pub fn is_add_tab_button_visible(mut self, visible: bool) -> Self {
        self.is_add_tab_button_visible = visible;
        self
    }

    pub fn on_selection_changed<F: Fn(i32) + 'static>(mut self, f: F) -> Self {
        self.on_selection_changed = Some(Callback::new(f));
        self
    }

    pub fn on_close_requested<F: Fn(String) + 'static>(mut self, f: F) -> Self {
        self.on_close_requested = Some(Callback::new(f));
        self
    }

    pub fn on_add_tab_button_click<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_add_tab_button_click = Some(Callback::new(move |()| f()));
        self
    }
}

impl Widget for TabView {
    widget_header!(ControlKind::TabView);

    fn bindings(&self) -> PropBindings {
        vec![
            Binding::Prop(Prop::SelectedIndex, PropValue::I32(self.selected_index)),
            Binding::Prop(
                Prop::CanReorderTabs,
                PropValue::Bool(self.can_reorder_tabs),
            ),
            Binding::Prop(
                Prop::IsAddTabButtonVisible,
                PropValue::Bool(self.is_add_tab_button_visible),
            ),
            Binding::Event(
                Event::SelectionChanged,
                self.on_selection_changed
                    .as_ref()
                    .map(|cb| EventHandler::I32(cb.clone())),
            ),
            Binding::Event(
                Event::CloseRequested,
                self.on_close_requested
                    .as_ref()
                    .map(|cb| EventHandler::Str(cb.clone())),
            ),
            Binding::Event(
                Event::AddTabButtonClick,
                self.on_add_tab_button_click
                    .as_ref()
                    .map(|cb| EventHandler::Unit(cb.clone())),
            ),
        ]
    }

    fn children(&self) -> Children<'_> {
        Children::Tabs(&self.tabs)
    }
}

pub fn tab_item(header: impl Into<String>, content: impl Into<Element>) -> TabItem {
    TabItem::new(header, content)
}

pub fn tab_view<I: IntoIterator<Item = TabItem>>(tabs: I) -> TabView {
    TabView::new(tabs)
}
