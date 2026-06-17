# Islands Reactor Project Handoff

Use this file as the starting context for a fresh agent conversation.

## Project Goal

`islands-reactor` is intended to be a port of upstream `windows-rs` Reactor, not
a new framework inspired by Reactor. The north star is that an application using
upstream Reactor can switch dependency/import names from Reactor to
`islands-reactor` / `islands_reactor` and keep running with little or no business
code change.

That means parity is larger than the public API. Preserve upstream Reactor's
public surface, internal module boundaries, folder structure, generated binding
style, event/prop semantics, gallery coverage, and runtime behavior wherever it
is technically possible.

The necessary divergence is the rendering/runtime kernel:

- Upstream Reactor is built around WinAppSDK / WinUI 3.
- `islands-reactor` replaces that kernel with `Windows.UI.Xaml` XAML Islands plus
  WinUI 2 (`Microsoft.UI.Xaml.Controls`, MUXC).
- Missing WinUI 3 behavior should be recreated on top of XAML Islands + WinUI 2
  when feasible, then documented when it cannot be made equivalent.

## Reference Repositories

- Upstream Reactor reference: `C:\Users\kimika\Desktop\rs-projects\references\windows-rs`
- Upstream Reactor crate: `C:\Users\kimika\Desktop\rs-projects\references\windows-rs\crates\libs\reactor`
- CoreIsland PRI/build reference: `C:\Users\kimika\Desktop\cs-projects\CoreIsland\CoreIsland\buildTransitive`
- XAML Islands backdrop reference: `C:\Users\kimika\Desktop\rs-projects\references\Xaml-Islands-Cpp\src\XamlIslandsCpp`

Read `docs/REACTOR_BASE.md` and `docs/UNSUPPORTED_REACTOR.md` before making
larger design changes.

## Workspace Shape

- Root `Cargo.toml` is only a workspace.
- Main library: `crates/islands-reactor`
- Setup/build helper library: `crates/islands-reactor-setup`
- Codegen tool: `crates/tools/islands-reactor-codegen`
- Samples: `samples/hello` and `samples/gallery`

Applications should depend on `islands-reactor` normally and use `islands-reactor-setup` as a `[build-dependencies]` crate. App build scripts call:

```rust
fn main() {
    islands_reactor_setup::embed_manifest();
    println!("cargo:rerun-if-changed=build.rs");
}
```

`islands-reactor-setup` stages WinUI 2 runtime assets, embeds the app manifest, and creates/merges the app `resources.pri`. It is a library crate; it should not need its own `build.rs`.

## Binding Policy

Normal builds must not run `windows-bindgen` and must not download from NuGet. Bindings are checked in under `crates/islands-reactor/src`:

- `bindings.rs`: private `Windows.UI.Xaml`/Win32/WinRT support bindings.
- `bindings_muxc.rs`: private WinUI 2/MUXC bindings.

Regeneration is explicit:

```powershell
cargo run -p islands-reactor-codegen -- generate-bindings
```

If a binding change is needed, update `crates/tools/islands-reactor-codegen/src/main.rs`, run the tool, and commit the generated result. Avoid long-term manual edits to generated binding files.

The MUXC metadata source is the `Microsoft.UI.Xaml.winmd` extracted from the WinUI 2 AppX beside the runtime DLL, not the `uap` folder metadata and not the .NET projection metadata.

## Porting Contract

For any feature, first inspect the upstream Reactor implementation in
`references/windows-rs/crates/libs/reactor`. Port the same Rust-facing API,
state shape, prop/event names, generated binding behavior, and file placement
where possible. Then replace only the platform-specific pieces needed to make it
work on XAML Islands + WinUI 2.

Prefer a faithful port with an Island-specific backend adapter over a
"similar-looking" wrapper. If upstream has a widget module, local code should
usually have the corresponding module. If upstream has generated prop/event
bindings, local code should preserve that model unless a clear technical blocker
exists. If upstream gallery has a page, local gallery should try to carry the
same page and behavior.

When the WinAppSDK / WinUI 3 implementation cannot be reproduced directly:

- Use `Windows.UI.Xaml` controls first when they match behavior.
- Use WinUI 2 MUXC controls when they fill a WinUI 3 control/API gap.
- Emulate behavior in the backend when neither WUX nor MUXC exposes it cleanly.
- Record remaining gaps in `docs/UNSUPPORTED_REACTOR.md`.

Concrete example: upstream Reactor exposes `button(...).accent()`. User code
should keep using `.accent()`. Internally, `islands-reactor` may map that variant
to the XAML resource `AccentButtonStyle`, but it should not make users write
`AccentButtonStyle` unless upstream Reactor also exposes that style-level API.

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

## Migration Target

The ideal downstream migration should look like this:

```toml
[dependencies]
islands-reactor = { path = "..." }
```

```rust
// before
use reactor::*;

// after
use islands_reactor::*;
```

The rest of the application code should stay as close as possible to the
upstream Reactor version. When that is not true, treat it as a parity bug or a
documented compatibility gap.

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
cargo run -p islands-reactor-codegen -- generate-bindings
git diff --check
```

After regeneration, rerunning the codegen command should produce no meaningful diff.

## Working Notes For The Next Agent

- Treat upstream Reactor as the blueprint for public API and internal structure.
- Before adding or changing an API, inspect upstream Reactor and port its shape.
- The project is private and fast-moving, so do not preserve local mistakes just
  for compatibility. Prefer getting closer to upstream Reactor.
- Avoid expensive binding regeneration unless the task actually needs it.
- If generated bindings must change, update codegen first, then regenerate.
- Keep commits focused and commit at natural checkpoints when asked.
- Keep XAML/MUXC bindings private implementation details. Users should consume
  `islands-reactor`, not raw binding modules.
