# bevy_easy_ui

[![CI](https://github.com/nvetroff/bevy_easy_ui/actions/workflows/ci.yml/badge.svg)](https://github.com/nvetroff/bevy_easy_ui/actions/workflows/ci.yml)
[![docs.rs](https://img.shields.io/docsrs/bevy_easy_ui)](https://docs.rs/bevy_easy_ui)
[![Crates.io](https://img.shields.io/crates/v/bevy_easy_ui)](https://crates.io/crates/bevy_easy_ui)
[![License](https://img.shields.io/crates/l/bevy_easy_ui)](LICENSE-MIT)
[![License](https://img.shields.io/crates/l/bevy_easy_ui)](LICENSE-APACHE)

A declarative, fluent builder-pattern abstraction layer on top of [Bevy 0.18](https://bevyengine.org/)'s UI system.

---

## What is it

`bevy_easy_ui` turns this:

```rust
commands.spawn((
    Button,
    Node { width: px(200.0), height: px(80.0), ..default() },
    BorderColor::all(Color::WHITE),
    BackgroundColor(Color::BLACK),
))
.with_children(|parent| {
    parent.spawn((
        Text::new("Click me!"),
        TextFont { font_size: 24.0, ..default() },
        TextColor(Color::WHITE),
    ));
});
```

…into this:

```rust
EasyButton::new()
    .with_border_color(EasyColor::WHITE)
    .with_border(px(2.0), px(10.0))
    .with_width(px(200.0))
    .with_height(px(80.0))
    .with_background_color(EasyColor::BLACK)
    .with_child(EasyLabel::new("Click me!").with_color(EasyColor::WHITE).with_font_size(24.0))
    .spawn(&mut commands);
```

Every setter is chainable, type-checked, and the trait system prevents misusing a container as a leaf (or vice versa).

---

## Quick start

```rust
use bevy::prelude::*;
use bevy_easy_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    EasyVerticalLayout::new()
        .with_z_index(1)
        .with_width(percent(100.0))
        .with_height(percent(100.0))
        .with_justify_content(JustifyContent::Center)
        .with_align_items(AlignItems::Center)
        .with_child(
            EasyButton::new()
                .with_z_index(2)
                .with_border_color(EasyColor::WHITE)
                .with_border(px(2.0), px(10.0))
                .with_background_color(EasyColor::BLACK)
                .with_child(EasyLabel::new("Click me!").with_color(EasyColor::WHITE).with_font_size(24.0).with_z_index(3))
        )
        .spawn(&mut commands);
}
```

That's the whole library. Run with `cargo run` and a centered dark button with white border and text appears.

---

## Examples

Each example is a standalone `cargo run --example NAME` showcasing a specific widget or pattern.

| Example | What it shows |
|---|---|
| `hello` | Minimal setup: a centered button with a label |
| `button_with_observers` | Buttons with `Pointer<Over>` / `Pointer<Out>` / `Pointer<Click>` observers (hover, click feedback) |
| `image_button` | Icon button built from `EasyButton` + `EasyImage` as a child |
| `rounded_image` | `EasyImage` with various `border_radius` values (sharp, small, full, asymmetric) |
| `scroll_vertical` | Scrollable `EasyVerticalLayout` using `with_overflow(Overflow::scroll())` |
| `carousel` | Horizontal scroll carousel using `with_overflow(Overflow::scroll_x())` |
| `viewport` | `EasyViewport` rendering a live camera output into a UI node |
| `rich_text` | `EasyRichText` with per-`EasySpan` colors, sizes, and justify |

```bash
cargo run --example hello
cargo run --example rich_text
```

---

## Widgets

| Widget | Bevy base | Purpose |
|---|---|---|
| `EasyVerticalLayout` | `Node` + `FlexDirection::Column` | Flex column container |
| `EasyHorizontalLayout` | `Node` + `FlexDirection::Row` | Flex row container |
| `EasyButton` | `Button` + `Node` | Clickable button (accepts children + observers) |
| `EasyText` | `Text` + `Node` + `TextFont` + `TextColor` | Styled text |
| `EasyLabel` | `Text` + `Node` + `Label` | Text marked as a label |
| `EasySpan` | `TextSpan` | Inline span used inside `EasyRichText` |
| `EasyRichText` | `Text` + `TextSpan` children | Multi-style text |
| `EasyImage` | `ImageNode` + `Node` | Image (rect, color, flip, mode, atlas) |
| `EasyViewport` | `Node` + `ViewportNode` | UI node displaying a `Camera` render target |

**Containers** (layouts, buttons, viewport) implement the `Container` trait and expose:
- `.with_child(impl Into<EasyElement>)` — adds a child
- `.with_observer(impl IntoObserverSystem)` — attaches a Bevy observer
- `.spawn(&mut Commands)` — finalizes and spawns the tree

---

## Reusable styles with `with_style`

Every widget also exposes a `with_style(style: <Widget>Style)` setter that swaps the **whole** style bundle at once — `Node` + `EasyBoxStyle` + `EasyStackStyle` (+ `EasyTextStyle` for text widgets). It's the same shape as a Bevy bundle, but assembled ahead of time.

Use it when you have a few pre-defined looks (e.g. a theme) you want to apply as a unit, without chaining ten `with_*` calls every time:

```rust
use bevy::prelude::*;
use bevy_easy_ui::prelude::*;

// Define a style once
fn primary_button_style() -> EasyButtonStyle {
    EasyButtonStyle {
        node: Node {
            width: px(200.0),
            height: px(64.0),
            padding: UiRect::all(px(12.0)),
            ..default()
        },
        box_style: EasyBoxStyle {
            background_color: BackgroundColor(EasyColor::BLUE),
            border_color: BorderColor::all(EasyColor::WHITE),
            ..default()
        },
        stack_style: EasyStackStyle {
            z_index: ZIndex(2),
            ..default()
        },
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    EasyVerticalLayout::new()
        .with_width(percent(100.0))
        .with_height(percent(100.0))
        .with_justify_content(JustifyContent::Center)
        .with_align_items(AlignItems::Center)
        .with_child(
            EasyButton::new()
                .with_style(primary_button_style())
                .with_child(
                    EasyLabel::new("Click me!")
                        .with_color(EasyColor::WHITE)
                        .with_font_size(24.0),
                ),
        )
        .spawn(&mut commands);
}
```

The available style types are: `EasyButtonStyle`, `EasyVerticalLayoutStyle`, `EasyHorizontalLayoutStyle`, `EasyRichTextStyle`, `EasyLabelStyle`, `EasyTextWidgetStyle`, `EasySpanStyle`, `EasyImageStyle`, `EasyViewportStyle`.

---

## The traits

Three extension traits add the builder setters on top of Bevy's components. They are implemented for every widget that owns the matching Bevy component, so the setters are always available.

### `EasyNode` — `Node` properties
Size (`with_width`, `with_height`, `with_min_*`, `with_max_*`), position (`with_position`, `with_top`, etc.), alignment (`with_align_items`, `with_justify_content`, …), spacing (`with_margin`, `with_padding`, `with_row_gap`, `with_column_gap`), borders (`with_border`, `with_border_radius`, `with_border_color` is in `EasyBoxStyleExt`), flex (`with_flex_direction`, `with_flex_wrap`, `with_flex_grow`, `with_flex_shrink`, `with_flex_basis`), grid (`with_grid_template_*`, `with_grid_auto_*`, `with_grid_row`, `with_grid_column`), overflow (`with_overflow`, `with_scrollbar_width`, `with_overflow_clip_margin`), display (`with_display`, `with_box_sizing`, `with_aspect_ratio`).

### `EasyBoxStyleExt` — background, border, shadow
`with_background_color`, `with_border_color`, `with_box_shadow`, `with_border_gradient`, `with_background_gradient`, `with_outline`.

### `EasyStackStyleExt` — z-index
`with_z_index`, `with_global_z_index`.

### `EasyTextStyleExt` — text-specific
`with_color` (text color), `with_font_family`, `with_font_size`, `with_font_weight`, `with_smoothing`, `with_features`, `with_justify`, `with_linebreak`, `with_line_height`, `with_text_shadow` / `with_shadow`.

### `EasyImageNode` — `ImageNode` properties
`with_image`, `with_image_color`, `with_texture_atlas`, `with_flip_x`, `with_flip_y`, `with_rect`, `with_image_mode`.

---

## Colors

`EasyColor` exposes constants for the common Bevy `Color`s (`WHITE`, `BLACK`) plus aliases (`DARK_GRAY`, `LIGHT_BLUE`, …). All values are `pub const Color`, so they slot directly into `with_background_color(EasyColor::BLUE)`.
However, you can create your own colors with `EasyColor::from_rgba()` using srgba() from Bevy.

---

## Contribution

Contributions are very welcome! Open an issue or a PR if you have any suggestions, questions, or want to add a new widget or feature.

---

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
