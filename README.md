# bevy_easy_ui

[![CI](https://github.com/nv-dev-labs/bevy_easy_ui/actions/workflows/ci.yml/badge.svg)](https://github.com/nv-dev-labs/bevy_easy_ui/actions/workflows/ci.yml)
[![docs.rs](https://img.shields.io/docsrs/bevy_easy_ui)](https://docs.rs/bevy_easy_ui)
[![Crates.io](https://img.shields.io/crates/v/bevy_easy_ui)](https://crates.io/crates/bevy_easy_ui)
[![License](https://img.shields.io/crates/l/bevy_easy_ui)](https://crates.io/crates/bevy_easy_ui#license)

A declarative, fluent builder-pattern abstraction layer on top of [Bevy 0.18](https://bevyengine.org/)'s UI system and [bevy_ui_text_input](https://crates.io/crates/bevy_ui_text_input)

## Version compatibility

| bevy_easy_ui | bevy_ui_text_input | bevy |
|---|---|---|
| 0.1.0 | 0.7.0 | 0.18.1 |

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
    .with_border_color(WHITE.into())
    .with_border(px(2.0), px(10.0))
    .with_width(px(200.0))
    .with_height(px(80.0))
    .with_background_color(BLACK.into())
    .with_child(
        EasyLabel::new("Click me!")
            .with_color(WHITE.into())
            .with_font_size(24.0),
    )
    .spawn(&mut commands);
```

Every setter is chainable, type-checked, and the trait system prevents misusing a container as a non-container (or vice versa).

---

## Quick start

```toml
# Cargo.toml
[dependencies]
bevy = "0.18.1"
bevy_easy_ui = "0.1.0"
```

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
                .with_border_color(WHITE.into())
                .with_border(px(2.0), px(10.0))
                .with_background_color(BLACK.into())
                .with_child(
                    EasyLabel::new("Click me!")
                        .with_z_index(3)
                        .with_color(WHITE.into())
                        .with_font_size(24.0),
                ),
        )
        .spawn(&mut commands);
}
```

Run with `cargo run` — a centered dark button with a white border and white text appears.

---

## Examples

Each example is a standalone `cargo run --example NAME` showcasing a specific widget or pattern.

| Example | What it shows |
|---|---|
| `hello` | Minimal setup: a centered button with a label |
| `button_with_observers` | Buttons with `Pointer<Over>` / `Pointer<Out>` / `Pointer<Click>` observers (hover, click feedback) |
| `image_button` | Icon button built from `EasyButton` + `EasyImage` as a child |
| `rounded_image` | `EasyImage` with various `border_radius` values (sharp, small, full, asymmetric) |
| `scroll` | Scrollable `EasyVerticalLayout` and `EasyHorizontalLayout` using `Overflow::scroll_y()` / `scroll_x()` + the `ScrollPlugin` mouse-wheel observer |
| `viewport` | `EasyViewport` rendering a live camera output into a UI node |
| `rich_text` | `EasyRichText` with per-`EasySpan` colors, sizes, and justify |
| `text_input` | `EasyTextInput` (re-export of `bevy_ui_text_input`) |

```bash
cargo run --example hello
cargo run --example rich_text
```

---

## Widgets

The crate ships a set of `Easy*` builders, each wrapping the matching Bevy component(s) with a fluent API.

| Widget | Bevy base | Kind | Purpose |
|---|---|---|---|
| `EasyVerticalLayout` | `Node` + `FlexDirection::Column` | container | Flex column layout |
| `EasyHorizontalLayout` | `Node` + `FlexDirection::Row` | container | Flex row layout |
| `EasyButton` | `Button` + `Node` | container | Clickable button (accepts children + observers) |
| `EasyRichText` | `Text` + `TextSpan` children | container | Multi-style text |
| `EasyLabel` | `Text` + `Node` + `Label` | non-container | Text marked as a label |
| `EasyText` | `Text` + `Node` + `TextFont` + `TextColor` | non-container | Styled text |
| `EasySpan` | `TextSpan` | non-container | Inline span used inside `EasyRichText` |
| `EasyImage` | `ImageNode` + `Node` | non-container | Image (rect, color, flip, mode, atlas) |
| `EasyTextInput` | `bevy_ui_text_input::TextInputNode` | non-container | Re-export of `bevy_ui_text_input` |
| `EasyViewport` | `Node` + `ViewportNode` | container | UI node displaying a `Camera` render target |

**Containers** (layouts, button, rich_text, viewport) implement the `Container` trait and expose:

- `.with_child(impl Into<EasyElement>)` — adds a child
- `.with_observer(impl IntoObserverSystem)` — attaches a Bevy observer
- `.spawn(&mut Commands)` — finalizes and spawns the tree

**Non-containers** (label, text, image, text_input) implement `WithObservers` and only expose `.with_observer(...)` and `.spawn(...)`.

---

## Reusable styles with `with_style`

Every widget also exposes a `with_style(style: <Widget>Style)` setter that swaps the **whole** style bundle at once — `Node` + `EasyBoxStyle` + `EasyStackStyle` (+ `EasyTextStyle` for text widgets). It's the same shape as a Bevy bundle, but assembled ahead of time.

Use it when you have a few pre-defined looks (e.g. a theme) you want to apply as a unit, without chaining ten `with_*` calls every time:

```rust
use bevy::prelude::*;
use bevy_easy_ui::prelude::*;

fn primary_button_style() -> EasyButtonStyle {
    EasyButtonStyle {
        node: Node {
            width: px(200.0),
            height: px(64.0),
            padding: UiRect::all(px(12.0)),
            ..default()
        },
        box_style: EasyBoxStyle {
            background_color: BackgroundColor(BLUE.into()),
            border_color: BorderColor::all(WHITE.into()),
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
                        .with_color(WHITE.into())
                        .with_font_size(24.0),
                ),
        )
        .spawn(&mut commands);
}
```

The available style types are: `EasyButtonStyle`, `EasyVerticalLayoutStyle`, `EasyHorizontalLayoutStyle`, `EasyRichTextStyle`, `EasyLabelStyle`, `EasyTextWidgetStyle`, `EasySpanStyle`, `EasyImageStyle`, `EasyViewportStyle`.

---

## The traits

Four extension traits add the builder setters on top of Bevy's components. They are implemented for every widget that owns the matching Bevy component, so the setters are always available.

### `EasyNode` — `Node` properties
Size (`with_width`, `with_height`, `with_min_*`, `with_max_*`), position (`with_position`, `with_top`, etc.), alignment (`with_align_items`, `with_justify_content`, …), spacing (`with_margin`, `with_padding`, `with_row_gap`, `with_column_gap`), borders (`with_border`, `with_border_radius`; `with_border_color` is in `EasyBoxStyleExt`), flex (`with_flex_direction`, `with_flex_wrap`, `with_flex_grow`, `with_flex_shrink`, `with_flex_basis`), grid (`with_grid_template_*`, `with_grid_auto_*`, `with_grid_row`, `with_grid_column`), overflow (`with_overflow`, `with_scrollbar_width`, `with_overflow_clip_margin`), display (`with_display`, `with_box_sizing`, `with_aspect_ratio`).

### `EasyBoxStyleExt` — background, border, shadow
`with_background_color`, `with_border_color`, `with_box_shadow`, `with_border_gradient`, `with_background_gradient`, `with_outline`.

### `EasyStackStyleExt` — z-index
`with_z_index`, `with_global_z_index`.

### `EasyTextStyleExt` — text-specific
`with_color` (text color), `with_font_family`, `with_font_size`, `with_font_weight`, `with_smoothing`, `with_features`, `with_justify`, `with_linebreak`, `with_line_height`, `with_text_shadow` / `with_shadow`.

### `EasyImageNode` — `ImageNode` properties
`with_image`, `with_image_color`, `with_texture_atlas`, `with_flip_x`, `with_flip_y`, `with_rect`, `with_image_mode`.

---

## Scrollable containers

The crate ships a tiny `ScrollPlugin` that turns the mouse wheel into a `Scroll` event you can attach to any `Overflow::scroll_*()` node:

```rust
use bevy::prelude::*;
use bevy_easy_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ScrollPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    EasyVerticalLayout::new()
        .with_width(px(320.0))
        .with_height(px(300.0))
        .with_overflow(Overflow::scroll_y())
        .with_padding(px(10.0))
        .with_child(EasyLabel::new("Top of the list"))
        // … more children …
        .spawn(&mut commands);
}
```

Hold `Ctrl` while scrolling to scroll horizontally instead.

---

## Colors

The prelude re-exports `bevy::color::palettes::css::*` (`WHITE`, `BLACK`, `BLUE`, `DARK_GRAY`, `LIGHT_BLUE`, …) so every color literal slots directly into a `with_*_color(...into())` call:

```rust
EasyButton::new()
    .with_background_color(BLUE.into())
    .with_border_color(WHITE.into())
```

If you need a custom color, build it with `bevy::color::Color::srgba(...)` and pass it through `.into()`.

---

## Contribution

Open an issue or a PR if you have suggestions, questions, or want to add a new widget or feature.

### Roadmap

The following widgets are planned but not yet wrapped as `Easy*` builders. They will be implemented by following the same pattern as the existing widgets, on top of the corresponding headless types in [`bevy_ui_widgets`](https://docs.rs/bevy_ui_widgets/0.18.1/):

- `EasyCheckbox` — wraps `bevy_ui_widgets::Checkbox` + `Checkable` + `Checked`
- `EasySlider` — wraps `bevy_ui_widgets::Slider` + `SliderValue` + `SliderRange` (+ optional `SliderStep` / `SliderPrecision`)
- `EasyRadioButton` + `EasyRadioGroup` — wraps `bevy_ui_widgets::RadioButton` + `RadioGroup`
- `EasyScrollbar` — wraps `bevy_ui_widgets::Scrollbar` + `CoreScrollbarThumb`

If you'd like to take one of these, the [integration checklist](#integration-checklist) below explains the wiring once the widget compiles.

### Adding a new widget

The crate follows a consistent pattern across all widgets — pick whichever existing widget is closest to what you want to build, then copy it:

1. **Bundle** — `#[derive(Bundle)] pub struct EasyXxx { ... pub node: Node, pub box_style: EasyBoxStyle, pub stack_style: EasyStackStyle }`. The bundle is the raw Bevy components grouped together.
2. **Container** — `pub struct EasyXxxContainer { bundle, children: Vec<EasyElement>, observers: Vec<Observer> }`. Holds the bundle plus any children/observers queued during building.
3. **Builder** — `EasyXxx::new() -> EasyXxxContainer` and `EasyXxx::default_bundle() -> Self`. `new()` always returns the container, never the bundle, so setters stay chainable.
4. **Accessor impls** — `EasyNode`, `EasyBoxStyleExt`, `EasyStackStyleExt` (and `EasyTextStyleExt` for text widgets). They expose the `with_*` setters.
5. **`Container` / `WithObservers` impl** — picks the right trait: `Container` if the widget can have children, `WithObservers` for non-containers.
6. **Style** — a `pub struct EasyXxxStyle { node, box_style, stack_style }` with `with_style(...)` on the builder, so users can swap the whole look at once.

### Integration checklist

Once the widget compiles, wire it into the rest of the crate so users find it under one import:

- Add a `pub mod` line in [`src/widgets/containers/mod.rs`](src/widgets/containers/mod.rs) (or `src/widgets/mod.rs` for non-container widgets).
- Add a variant `EasyXxxContainer(EasyXxxContainer)` in [`src/core/element.rs`](src/core/element.rs) and a matching `From<EasyXxxContainer> for EasyElement` impl.
- Re-export the bundle, container, and style with `pub use ...::*;` in [`src/prelude.rs`](src/prelude.rs).
- Add a `cargo run --example xxx` example in [`examples/`](examples/) and reference it in the **Examples** table of this README.

### Filing issues

For bug reports, include the Bevy version, the crate version, a minimal repro, and what you expected vs. what you got. For feature requests, sketch the API you'd like to call — `EasyXxx::new().with_*(...).with_child(...).spawn(&mut commands)` is the shape we aim for.

---

## Known limitations

This is a 0.1.0 release — the API works and is covered by the eight examples, but it is still a young library with rough edges. Things will move, names will change, and some patterns may not be fully fleshed out yet. Contributions and bug reports are very welcome, and feedback from early users is the fastest way to make the next version better.

If you hit something unexpected, please open an issue — even small reports help prioritize what to harden next.

---

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
