use crate::controls::*;
use islands_reactor::*;

pub fn progress_ring_page(_: &(), cx: &mut RenderCx) -> Element {
    let (active, set_active) = cx.use_state(true);

    page_content(
        "ProgressRing",
        "A circular indicator of ongoing progress.",
        vec![
            sample_card(
                "Indeterminate ProgressRing",
                ProgressRing::indeterminate(),
                "ProgressRing::indeterminate()",
            ),
            sample_card(
                "Active state",
                vstack((
                    ProgressRing::new().active(active),
                    ToggleSwitch::new(active).on_toggled(move |v| set_active.call(v)),
                ))
                .spacing(8.0),
                "ProgressRing::new().active(is_active)",
            ),
        ],
    )
}
