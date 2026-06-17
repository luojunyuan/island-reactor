#![windows_subsystem = "windows"]

use islands_reactor::*;

mod controls;
mod pages;
mod registry;
mod router;
mod shell;

fn main() {
    let _ = App::new()
        .title("Islands Reactor Gallery")
        .inner_size(1400.0, 900.0)
        .backdrop(Backdrop::Mica)
        .eager_templated_realization(true)
        .render(shell::gallery_shell);
}
