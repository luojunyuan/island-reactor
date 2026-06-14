#![windows_subsystem = "windows"]

use island_reactor::*;

fn main() -> Result<()> {
    App::new().title("Rust Island Reactor Hello").render(app)
}

fn app(_cx: &mut RenderCx) -> Element {
    vstack((
        text_block("Hello, Rust Island Reactor!")
            .font_size(24.0)
            .bold(),
        text_block("This is a native Windows.UI.Xaml Islands app built from Rust."),
    ))
    .spacing(8.0)
    .padding(24.0)
    .into()
}
