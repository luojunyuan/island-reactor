#![windows_subsystem = "windows"]

use island_reactor::*;

mod controls;
mod pages;
mod registry;
mod router;
mod shell;

fn main() {
    let _ = App::new()
        .title("Island Reactor Gallery")
        .inner_size(1400.0, 900.0)
        .eager_templated_realization(true)
        .render(shell::gallery_shell);
}
