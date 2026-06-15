use crate::controls::*;
use island_reactor::*;

pub fn radio_button_page(_: &(), cx: &mut RenderCx) -> Element {
    let (selected, set_selected) = cx.use_state(0_i32);
    let (buttons_selected, set_buttons_selected) = cx.use_state(0_i32);

    let options = ["Option A", "Option B", "Option C"];
    let label = options.get(selected as usize).copied().unwrap_or("?");
    let delivery = ["Email", "SMS", "None"];
    let delivery_label = delivery
        .get(buttons_selected as usize)
        .copied()
        .unwrap_or("(none)");

    page_content(
        "RadioButton",
        "Select one option from a group.",
        vec![
            sample_card(
                "Grouped RadioButton controls",
                vstack((
                    text_block("Pick one").semibold(),
                    RadioButton::new(options[0])
                        .group("gallery-radio-options")
                        .checked(selected == 0)
                        .on_checked({
                            let set_selected = set_selected.clone();
                            move || set_selected.call(0)
                        }),
                    RadioButton::new(options[1])
                        .group("gallery-radio-options")
                        .checked(selected == 1)
                        .on_checked({
                            let set_selected = set_selected.clone();
                            move || set_selected.call(1)
                        }),
                    RadioButton::new(options[2])
                        .group("gallery-radio-options")
                        .checked(selected == 2)
                        .on_checked({
                            let set_selected = set_selected;
                            move || set_selected.call(2)
                        }),
                    text_block(format!("Selected: {label}")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"RadioButton::new("Option A").group("group").checked(selected == 0)"#,
            ),
            sample_card(
                "RadioButtons",
                vstack((
                    RadioButtons::new(delivery)
                        .header("Notifications")
                        .selected_index(buttons_selected)
                        .max_columns(3)
                        .on_selection_changed(move |i| set_buttons_selected.call(i)),
                    text_block(format!("Selected: {delivery_label}")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"RadioButtons::new(["Email", "SMS", "None"]).selected_index(index)"#,
            ),
            sample_card(
                "Disabled RadioButton",
                RadioButton::new("Unavailable option")
                    .group("disabled-radio")
                    .enabled(false),
                r#"RadioButton::new("Unavailable option").enabled(false)"#,
            ),
        ],
    )
}
