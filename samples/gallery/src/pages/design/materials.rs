use crate::controls::{page_content, sample_card};
use islands_reactor::*;

pub fn materials_page(_: &(), cx: &mut RenderCx) -> Element {
    let (selected, set_selected) = cx.use_state(0_i32);

    let options: Vec<String> = ["Mica", "Mica Alt", "Acrylic", "None (solid)"]
        .into_iter()
        .map(String::from)
        .collect();

    let description = match selected {
        0 => "Mica is the default backdrop for primary app windows.",
        1 => "Mica Alt uses a stronger tint for tabbed or sectioned windows.",
        2 => "Acrylic uses the transient window backdrop material on supported Windows builds.",
        _ => "Solid removes the DWM system backdrop from this window.",
    };

    page_content(
        "Materials",
        "Window backdrop materials that provide depth behind islands-reactor content.",
        vec![
            sample_card(
                "Live Backdrop Switcher",
                vstack((
                    list_view(options, |item, _idx| {
                        text_block(item.clone()).padding(Thickness::uniform(10.0))
                    })
                    .with_key_selector(|item| item.clone())
                    .selected_index(selected)
                    .on_selection_changed({
                        let set_selected = set_selected;
                        move |index: i32| {
                            let backdrop = match index {
                                0 => Some(Backdrop::Mica),
                                1 => Some(Backdrop::MicaAlt),
                                2 => Some(Backdrop::Acrylic),
                                _ => None,
                            };
                            set_backdrop(backdrop);
                            set_selected.call(index);
                        }
                    })
                    .height(170.0),
                    text_block(description)
                        .foreground(ThemeRef::SecondaryText)
                        .wrap(),
                ))
                .spacing(8.0),
                r#"set_backdrop(Some(Backdrop::Mica));
set_backdrop(Some(Backdrop::MicaAlt));
set_backdrop(Some(Backdrop::Acrylic));
set_backdrop(None);"#,
            ),
            sample_card(
                "App Startup",
                vstack((
                    text_block("Use App::backdrop before run or render to choose the initial window material."),
                    text_block("The window is created with the DWM style needed for Mica/Acrylic on Windows 11 22H2 and newer.")
                        .foreground(ThemeRef::SecondaryText)
                        .wrap(),
                ))
                .spacing(8.0),
                r#"App::new()
    .backdrop(Backdrop::Mica)
    .render(app);"#,
            ),
        ],
    )
}
