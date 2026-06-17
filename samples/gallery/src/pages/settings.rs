use islands_reactor::*;

pub fn settings_page(_: &(), _cx: &mut RenderCx) -> Element {
    let content: Element = vstack((
        text_block("Settings").font_size(28.0).bold(),
        vstack((
            text_block("About this app").bold(),
            vstack((
                text_block("Islands Reactor Gallery").bold(),
                text_block("Version 0.1.0").font_size(12.0).opacity(0.6),
            ))
            .spacing(2.0),
            text_block(
                "This app is built with Islands Reactor, a declarative component-based UI framework for Windows.UI.Xaml XAML Islands."
            )
            .font_size(13.0)
            .wrap()
            .opacity(0.6)
        ))
        .spacing(12.0)
        .padding(20.0),
        vstack((
            text_block("Built with Reactor").bold(),
            text_block("Framework: Reactor (declarative Rust DSL)").font_size(13.0),
            text_block("Platform: Windows.UI.Xaml XAML Islands").font_size(13.0),
            text_block("Rendering: Virtual DOM reconciler").font_size(13.0),
            text_block("State: Hook-based state management").font_size(13.0)
        ))
        .spacing(8.0)
        .padding(20.0)
    ))
    .spacing(24.0)
    .margin(Thickness {
        left: 36.0,
        top: 24.0,
        right: 36.0,
        bottom: 36.0,
    })
    .horizontal_alignment(HorizontalAlignment::Stretch)
    .into();
    scroll_viewer(content).into()
}
