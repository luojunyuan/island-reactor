# Reactor Base

This crate ports the upstream `windows-rs` Reactor crate, targeting broad
behavioral and structural parity rather than a small subset of its public API.

`islands-reactor` should preserve Reactor's public API, module and folder
structure, generated binding model, and runtime behavior wherever practical.
The core work replaces the WinAppSDK / WinUI 3 kernel with
`Windows.UI.Xaml` XAML Islands and WinUI 2 (`Microsoft.UI.Xaml` / MUXC), then
closes the remaining behavioral gaps so Reactor applications can migrate by
changing their package/import name.
