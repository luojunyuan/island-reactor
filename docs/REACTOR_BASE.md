# Reactor Base

This crate is a port of the upstream `windows-rs` Reactor crate at this
reference point. The target is broad behavioral and structural parity, not just
matching a handful of public functions:

- Repository path: `C:\Users\kimika\Desktop\rs-projects\references\windows-rs`
- Reactor crate path: `crates\libs\reactor`
- Commit: `62af965b827a68079605ee6461241764e4a6374b`
- Commit date: `2026-06-10T07:57:37-05:00`
- Commit Unix timestamp: `1781096257`
- Commit subject: `Refresh docs (#4554)`
- Reference worktree status when recorded: clean
- Recorded date: `2026-06-15`

`island-reactor` should preserve Reactor's public API, internal module shape,
folder layout, generated binding model, and runtime behavior wherever that is
practical. The core porting work is replacing Reactor's WinAppSDK / WinUI 3
kernel with `Windows.UI.Xaml` XAML Islands plus WinUI 2 (`Microsoft.UI.Xaml`
/ MUXC), then filling the behavioral gaps until existing Reactor applications
can run after changing the package/import name.
