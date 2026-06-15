#![windows_subsystem = "windows"]

use island_reactor::*;

fn main() -> Result<()> {
    App::new()
        .title("Island Reactor NavigationView Smoke")
        .render(app)
}

fn app(cx: &mut RenderCx) -> Element {
    let (selected, set_selected) = cx.use_state(String::from("home"));
    let items = [
        NavViewItem::new("Home").tag("home").icon(SymbolGlyph::Home),
        NavViewItem::new("Browse")
            .tag("browse")
            .icon(SymbolGlyph::Find),
        NavViewItem::new("Settings")
            .tag("settings")
            .icon(SymbolGlyph::Setting),
    ];

    let content: Element = match selected.as_str() {
        "browse" => text_block("Browse selected").into(),
        "settings" => text_block("Settings selected").into(),
        _ => text_block("Home selected").into(),
    };

    NavigationView::new(items, content)
        .selected_tag(&selected)
        .on_selection_changed(move |tag: String| set_selected.call(tag))
        .pane_title("Smoke")
        .settings_visible(false)
        .into()
}
