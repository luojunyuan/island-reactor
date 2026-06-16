# Island Reactor Project Handoff

Use this file as the starting context for a fresh agent conversation.

## Project Goal

`island-reactor` is a Rust UI library that follows the public API shape of upstream `windows-rs` Reactor while hosting `Windows.UI.Xaml` XAML Islands and WinUI 2 (`Microsoft.UI.Xaml.Controls`) runtime assets. The goal is not to invent a new UI framework surface. The goal is to keep the public wrapper API aligned with upstream Reactor wherever practical, and document every intentional difference.

## Reference Repositories

- Upstream Reactor reference: `C:\Users\kimika\Desktop\rs-projects\references\windows-rs`
- Upstream Reactor crate: `C:\Users\kimika\Desktop\rs-projects\references\windows-rs\crates\libs\reactor`
- CoreIsland PRI/build reference: `C:\Users\kimika\Desktop\cs-projects\CoreIsland\CoreIsland\buildTransitive`
- XAML Islands backdrop reference: `C:\Users\kimika\Desktop\rs-projects\references\Xaml-Islands-Cpp\src\XamlIslandsCpp`

Read `docs/REACTOR_BASE.md` and `docs/UNSUPPORTED_REACTOR.md` before making larger design changes.

## Workspace Shape

- Root `Cargo.toml` is only a workspace.
- Main library: `crates/island-reactor`
- Setup/build helper library: `crates/island-reactor-setup`
- Codegen tool: `crates/tools/island-reactor-codegen`
- Samples: `samples/hello` and `samples/gallery`

Applications should depend on `island-reactor` normally and use `island-reactor-setup` as a `[build-dependencies]` crate. App build scripts call:

```rust
fn main() {
    island_reactor_setup::embed_manifest();
    println!("cargo:rerun-if-changed=build.rs");
}
```

`island-reactor-setup` stages WinUI 2 runtime assets, embeds the app manifest, and creates/merges the app `resources.pri`. It is a library crate; it should not need its own `build.rs`.

## Binding Policy

Normal builds must not run `windows-bindgen` and must not download from NuGet. Bindings are checked in under `crates/island-reactor/src`:

- `bindings.rs`: private `Windows.UI.Xaml`/Win32/WinRT support bindings.
- `bindings_muxc.rs`: private WinUI 2/MUXC bindings.

Regeneration is explicit:

```powershell
cargo run -p island-reactor-codegen -- generate-bindings
```

If a binding change is needed, update `crates/tools/island-reactor-codegen/src/main.rs`, run the tool, and commit the generated result. Avoid long-term manual edits to generated binding files.

The MUXC metadata source is the `Microsoft.UI.Xaml.winmd` extracted from the WinUI 2 AppX beside the runtime DLL, not the `uap` folder metadata and not the .NET projection metadata.

## API Alignment Rules

- Prefer the upstream Reactor public API exactly when it exists.
- Do not add convenience APIs that upstream Reactor does not have unless the user explicitly asks to intentionally diverge.
- When adapting behavior because this project uses XAML Islands + WinUI 2 rather than WinUI 3/Windows App SDK, document the difference in `docs/UNSUPPORTED_REACTOR.md`.
- Keep XAML/MUXC bindings private implementation details. Users should consume `island-reactor`, not raw binding crates or modules.
- If a feature can be implemented by wrapping MUXC while preserving Reactor's API shape, do that.

Example: upstream Reactor supports `button(...).accent()`, not `.style(ThemeRef::custom("AccentButtonStyle"))`. The backend may map `ButtonStyle::Accent` to the `AccentButtonStyle` resource internally, but the public API should remain `.accent()`.

## Current Important Implementation Notes

- Button style variants should use XAML style resources from `Application.Current.Resources`:
  - `ButtonStyle::Accent` -> `AccentButtonStyle`
  - `ButtonStyle::Subtle` -> `SubtleButtonStyle`
  - `ButtonStyle::TextLink` -> `TextBlockButtonStyle`
- Theme brush bindings use dynamically loaded XAML `Style` setters with `{ThemeResource ...}`, matching the upstream style-based approach. Example user code:

```rust
border(content)
    .background(ThemeRef::custom("ApplicationPageBackgroundThemeBrush"))
```

- Backdrop support follows the Xaml-Islands-Cpp DWM path, not CoreIsland and not upstream WinAppSDK `SystemBackdrop`.
- Host windows use `WS_EX_NOREDIRECTIONBITMAP`.
- For Mica/Acrylic to show through a XAML Island, root backgrounds often need transparent or theme-resource-aware handling.

## Verification Commands

Run the smallest useful verification first, then widen if the change touches shared behavior:

```powershell
cargo fmt --workspace
cargo check --workspace --bins
cargo test --workspace
cargo build --workspace --bins
git diff --check
```

For binding/tooling changes:

```powershell
cargo run -p island-reactor-codegen -- generate-bindings
git diff --check
```

After regeneration, rerunning the codegen command should produce no meaningful diff.

## Communication Notes For The Next Agent

- The user strongly prefers upstream Reactor compatibility over ad hoc APIs.
- The project is still private and fast-moving, so clean design beats backward compatibility.
- Avoid expensive binding regeneration unless the task actually needs it.
- If generated bindings must change, explain why, update codegen first, then regenerate.
- Keep commits focused and commit at natural checkpoints when asked.
- When unsure whether an API exists upstream, inspect `references/windows-rs` before adding anything.
