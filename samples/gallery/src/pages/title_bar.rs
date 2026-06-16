use crate::controls::{page_content, sample_card};
use island_reactor::*;

pub fn title_bar_page(_: &(), cx: &mut RenderCx) -> Element {
    let (message, set_message) = cx.use_state(String::from("No title bar command yet."));
    let (tall, set_tall) = cx.use_state(false);
    let (show_back, set_show_back) = cx.use_state(true);
    let (show_pane, set_show_pane) = cx.use_state(true);

    page_content(
        "TitleBar",
        "A title bar control for app chrome, custom content, and navigation affordances.",
        vec![
            sample_card(
                "Title and Subtitle",
                TitleBar::new("Island Reactor")
                    .subtitle("Windows.UI.Xaml host")
                    .content(text_block("Center content").opacity(0.72))
                    .footer(
                        text_block("Footer")
                            .opacity(0.72)
                            .padding(Thickness::xy(12.0, 0.0)),
                    ),
                r#"TitleBar::new("Island Reactor")
    .subtitle("Windows.UI.Xaml host")
    .content(text_block("Center content"))
    .footer(text_block("Footer"))"#,
            ),
            sample_card(
                "Back and Pane Toggle",
                vstack((
                    TitleBar::new("Navigation")
                        .subtitle("BackRequested and PaneToggleRequested")
                        .back_button_visible(show_back)
                        .back_button_enabled(show_back)
                        .pane_toggle_button_visible(show_pane)
                        .on_back_requested({
                            let set_message = set_message.clone();
                            move || set_message.call(String::from("BackRequested"))
                        })
                        .on_pane_toggle_requested({
                            let set_message = set_message.clone();
                            move || set_message.call(String::from("PaneToggleRequested"))
                        }),
                    hstack((
                        ToggleSwitch::new(show_back)
                            .header("Back button")
                            .on_toggled({
                                let set_show_back = set_show_back;
                                move |value| set_show_back.call(value)
                            }),
                        ToggleSwitch::new(show_pane)
                            .header("Pane toggle")
                            .on_toggled({
                                let set_show_pane = set_show_pane;
                                move |value| set_show_pane.call(value)
                            }),
                    ))
                    .spacing(24.0),
                    text_block(message).foreground(ThemeRef::SecondaryText),
                ))
                .spacing(12.0),
                r#"TitleBar::new("Navigation")
    .back_button_visible(true)
    .back_button_enabled(true)
    .pane_toggle_button_visible(true)
    .on_back_requested(|| { /* ... */ })
    .on_pane_toggle_requested(|| { /* ... */ })"#,
            ),
            sample_card(
                "Tall TitleBar",
                vstack((
                    TitleBar::new("Tall title bar")
                        .subtitle("Use tall(true) for the taller chrome layout")
                        .tall(tall)
                        .content(text_block(if tall { "Tall" } else { "Standard" }).opacity(0.72)),
                    ToggleSwitch::new(tall)
                        .header("Tall")
                        .on_toggled(move |value| set_tall.call(value)),
                ))
                .spacing(12.0),
                r#"TitleBar::new("Tall title bar")
    .subtitle("Use tall(true) for the taller chrome layout")
    .tall(true)"#,
            ),
        ],
    )
}
