#![windows_subsystem = "windows"]

use island_reactor::*;

fn main() -> Result<()> {
    App::new()
        .title("Rust Island Reactor Hello")
        .backdrop(Backdrop::Mica)
        .render(app)
}

fn app(_cx: &mut RenderCx) -> Element {
    let mut layout = grid(())
        .rows([GridLength::Auto, GridLength::Auto, GridLength::Auto])
        .row_spacing(8.0)
        .width(190.0);
    layout.children = vec![
        text_block("Hello, island-reactor")
            .font_size(20.0)
            .bold()
            .wrap()
            .grid_row(0)
            .into(),
        text_block("WinUI 2 package styles loaded.")
            .wrap()
            .grid_row(1)
            .into(),
        button("WinUI 2 Button")
            .accent()
            .horizontal_alignment(HorizontalAlignment::Right)
            .grid_row(2)
            .into(),
    ];
    border(layout)
        .padding(16.0)
        .background(Color {
            a: 0,
            r: 0,
            g: 0,
            b: 0,
        })
        .into()
}
