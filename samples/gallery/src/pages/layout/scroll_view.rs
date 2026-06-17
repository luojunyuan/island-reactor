use crate::controls::*;
use islands_reactor::*;

pub fn scroll_view_page(_: &(), _cx: &mut RenderCx) -> Element {
    let rows: Vec<Element> = (1..=18)
        .map(|i| {
            border(text_block(format!("Scrollable row {i}")))
                .padding(Thickness::uniform(12.0))
                .background(ThemeRef::SubtleFill)
                .corner_radius(6.0)
                .into()
        })
        .collect();

    page_content(
        "ScrollView",
        "A scrollable container for overflowing content. In the current WinUI2 AppX winmd profile this sample is backed by Windows.UI.Xaml ScrollViewer.",
        vec![sample_card(
            "Vertical Scrolling",
            scroll_viewer(vstack(rows).spacing(8.0))
                .height(240.0)
                .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto),
            r#"scroll_viewer(content).vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)"#,
        )],
    )
}
