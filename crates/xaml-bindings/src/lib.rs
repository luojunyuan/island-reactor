#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/xaml_bindings.rs"));
}

pub use generated::Windows;
