use crate::controls::*;
use islands_reactor::*;

pub fn title_bar_page(_: &(), cx: &mut RenderCx) -> Element {
    let (status, set_status) = cx.use_state(String::from("Ready"));
    let title_bar = TitleBar::new("Islands Writer")
        .subtitle("Draft")
        .back_button_visible(true)
        .back_button_enabled(true)
        .pane_toggle_button_visible(true)
        .on_back_requested({
            let set_status = set_status.clone();
            move || set_status.call(String::from("Back requested"))
        })
        .on_pane_toggle_requested(move || set_status.call(String::from("Pane toggle requested")))
        .content(auto_suggest_box("").placeholder_text("Search").width(260.0))
        .footer(button("Share").subtle())
        .tall(true);

    page_content(
        "TitleBar",
        "A customizable application title bar.",
        vec![sample_card(
            "Custom TitleBar",
            vstack((
                title_bar,
                text_block(status.clone()).foreground(ThemeRef::SecondaryText),
            ))
            .spacing(12.0),
            r#"TitleBar::new("Islands Writer")
    .subtitle("Draft")
    .back_button_visible(true)
    .pane_toggle_button_visible(true)
    .content(auto_suggest_box("").placeholder_text("Search"))
    .footer(button("Share").subtle())"#,
        )],
    )
}
