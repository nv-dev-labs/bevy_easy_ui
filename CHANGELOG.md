# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

- Added `EasySlider` + `EasySliderThumb` builders for the `bevy_ui_widgets::Slider` widget
- Added `Slider` example showcasing a horizontal slider with a thumb that updates its position based on a `ValueChange<f32>` observer and with a label showing the current value dynamically.
- Removed `with_text_shadow`, only `with_shadow` remains.

## [0.1.1] - 2026-06-10

- Modified params for `with_border_radius` and `with_border` to accept a single `Val` instead of 4 separate values, and apply it to all sides.
- Added variants for `with_border_color`, `with_border_width`, `with_border_radius` that accepts a single value and applies it to the border or corner chosen by the method name (e.g. `with_border_radius_top_right`).

## [0.1.0] - 2026-06-10

### Added

Initial release of `bevy_easy_ui`, a fluent builder-pattern abstraction layer on top of Bevy 0.18.1's UI system.

**Widgets — containers**

- `EasyVerticalLayout` — `Node` + `FlexDirection::Column` flex column.
- `EasyHorizontalLayout` — `Node` + `FlexDirection::Row` flex row.
- `EasyButton` — `Button` + `Node` clickable button (accepts children + observers).
- `EasyRichText` — `Text` + per-`EasySpan` children, multi-style text.

**Widgets — non-containers**

- `EasyText` — `Text` + `TextFont` + `TextColor` styled text.
- `EasyLabel` — `Text` + `Node` + `Label` (text marked as a label).
- `EasySpan` — `TextSpan` inline span used inside `EasyRichText`.
- `EasyImage` — `ImageNode` + `Node` image (rect, color, flip, mode, atlas).
- `EasyTextInput` — re-export of `bevy_ui_text_input::TextInputNode`.
- `EasyViewport` — `Node` + `ViewportNode` (UI node displaying a `Camera` render target).

**Extension traits** (expose the `with_*` builder setters)

- `EasyNode` — `Node` properties (size, position, alignment, spacing, borders, flex, grid, overflow, display).
- `EasyBoxStyleExt` — background, border, shadow, gradient, outline.
- `EasyStackStyleExt` — `with_z_index`, `with_global_z_index`.
- `EasyTextStyleExt` — `with_color`, `with_font_family`, `with_font_size`, `with_font_weight`, `with_smoothing`, `with_features`, `with_justify`, `with_linebreak`, `with_line_height`, `with_text_shadow`.
- `EasyImageNode` — `ImageNode` properties (`with_image`, `with_image_color`, `with_texture_atlas`, `with_flip_x`, `with_flip_y`, `with_rect`, `with_image_mode`).

**Container API**

- `Container` trait — bundles + children (`Vec<EasyElement>`) + observers (`Vec<Observer>`), with `.with_child(...)`, `.with_observer(...)`, and `.spawn(&mut Commands)`.
- `WithObservers` trait — same API for non-containers (no children).
- `EasyElement` enum — sum type holding every widget variant, with `From` impls for seamless `.with_child(...)` chaining.
- `with_style(...)` on every widget — swap the whole `Node` + `EasyBoxStyle` + `EasyStackStyle` (+ `EasyTextStyle` for text widgets) at once.

**Mouse-wheel scrolling**

- `ScrollPlugin` — turns the mouse wheel into a `Scroll` event and propagates it to hovered nodes.
- `on_scroll_handler` observer — applies the delta to a node's `ScrollPosition` (vertical by default, horizontal when `Ctrl` is held). Attach it to any `Overflow::scroll_y()` / `scroll_x()` node via `.with_observer(on_scroll_handler)`.

**Re-exports in the prelude**

- `bevy::color::palettes::css::*` — `WHITE`, `BLACK`, `BLUE`, `DARK_GRAY`, `LIGHT_BLUE`, … usable directly in `with_*_color(...into())` calls.
- `bevy::text::{FontFeatures, FontSmoothing}`.
- `bevy_ui_text_input::TextInputPlugin` — drop in to enable `EasyTextInput` focus / IME.

**Examples**

Runnable demos in `examples/`, each showcases a specific widget or pattern:

- `hello` — minimal setup: a centered button with a label.
- `button_with_observers` — `Pointer<Over>` / `Pointer<Out>` / `Pointer<Click>` observers (hover, click feedback).
- `image_button` — icon button built from `EasyButton` + `EasyImage` as a child.
- `rounded_image` — `EasyImage` with various `border_radius` values (sharp, small, full, asymmetric).
- `scroll` — scrollable `EasyVerticalLayout` and `EasyHorizontalLayout` using `Overflow::scroll_y()` / `scroll_x()` + `ScrollPlugin`.
- `viewport` — `EasyViewport` rendering a live camera output into a UI node.
- `rich_text` — `EasyRichText` with per-`EasySpan` colors, sizes, and justify.
- `text_input` — `EasyTextInput` (re-export of `bevy_ui_text_input`).

**License**

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
