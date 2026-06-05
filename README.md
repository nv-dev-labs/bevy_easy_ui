# bevy_easy_ui

A declarative, fluent abstraction layer (builder pattern) on top of [Bevy 0.18](https://bevyengine.org/)'s UI system, designed to build interfaces without having to manually juggle `Bundle`s, `Entity` ids, and `ChildBuilder` calls every time.

---

## Goal

With the standard Bevy UI API, creating a simple styled button containing some text requires chaining several `commands.spawn(...)`, `with_children(...)` and `insert(...)` calls. `bevy_easy_ui` offers a **declarative** and **chainable** alternative:

```rust
// Classic Bevy API
commands.spawn((
    Button,
    Node { width: px(200.0), height: px(80.0), ..default() },
    BorderColor::all(EasyColor::WHITE),
    BackgroundColor(EasyColor::BLACK),
))
.with_children(|parent| {
    parent.spawn((
        Text::new("Click me!"),
        TextFont { font_size: 24.0, ..default() },
        TextColor(EasyColor::WHITE),
    ));
});
```

```rust
// With bevy_easy_ui
EasyButton::new()
    .border_color(EasyColor::WHITE)
    .border(px(2.0), px(10.0))
    .width(px(200.0))
    .height(px(80.0))
    .child(
        EasyLabel::new("Click me!")
            .color(EasyColor::WHITE)
            .font_size(24.0)
    )
    .spawn(&mut commands);
```

---

## Dependency

Add the crate to your `Cargo.toml`. **The version of `bevy_easy_ui` must match the version of `bevy` you depend on** — see the [Versioning](#versioning) section above.

```toml
[dependencies]
bevy = "0.18.1"
bevy_easy_ui = "0.18.1"
```

> The crate only depends on `bevy`. No additional external dependencies.

---

## Quick Start

### 1. Import the prelude

```rust
use bevy::prelude::*;
use bevy_easy_ui::prelude::*;
```

The `prelude` re-exports the widgets and helpers (colors) ready to use.

### 2. Build a UI tree

The API follows a three-step pattern:

1. **Pick a root container** (e.g. `EasyVerticalLayout::new()`).
2. **Chain setters** through the methods of the `EasyNode` trait (size, margins, padding, alignment, flex, etc.).
3. **Add children** with `.child(...)` and finish with `.spawn(&mut commands)`.

### Full example

A screen made of a centered button above a horizontal row containing a button and a text:

````rust
use bevy::prelude::*;
use bevy_easy_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn hover_in(hover_in: On<Pointer<Over>>, mut commands: Commands) {
    commands.entity(hover_in.entity).insert(BackgroundColor(EasyColor::DARK_BLUE));
}

fn hover_out(hover_out: On<Pointer<Out>>, mut commands: Commands) {
    commands.entity(hover_out.entity).insert(BackgroundColor(EasyColor::BLACK));
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    EasyVerticalLayout::new()
        .width(percent(100.0))
        .height(percent(100.0))
        .justify_content(JustifyContent::Center)
        .align_items(AlignItems::Center)
        .child(
            EasyButton::new()
                .border_color(EasyColor::WHITE)
                .border(px(2.0), px(10.0))
                .margin(px(0.0), px(0.0), px(20.0), px(0.0))
                .width(px(200.0))
                .height(px(80.0))
                .observe(hover_in)
                .observe(hover_out)
                .child(
                    EasyLabel::new("Click me!")
                        .color(EasyColor::WHITE)
                        .font_size(24.0)
                )
        )
        .child(
            EasyHorizontalLayout::new()
                .justify_content(JustifyContent::Center)
                .align_items(AlignItems::Center)
                .child(
                    EasyButton::new()
                        .border_color(EasyColor::WHITE)
                        .border(px(2.0), px(10.0))
                        .width(px(150.0))
                        .height(px(60.0))
                        .observe(hover_in)
                        .observe(hover_out)
                        .child(
                            EasyLabel::new("To the left !")
                                .color(EasyColor::WHITE)
                                .font_size(18.0)
                        )
                )
                .child(
                    EasyText::new("To the right !")
                        .color(EasyColor::WHITE)
                        .font_size(18.0)
                )
        )
        .spawn(&mut commands);
}
````

---

## Provided Widgets

| Widget | Description |
|---|---|
| `EasyVerticalLayout` | Flex **column** container (`FlexDirection::Column`) |
| `EasyHorizontalLayout` | Flex **row** container (`FlexDirection::Row`) |
| `EasyButton` | Clickable button (Bundle `Button + Node + BorderColor + BackgroundColor`) |
| `EasyText` | Styled text label (Bundle `Text + Node + TextFont + TextColor + TextShadow`) |

**Containers** (vertical/horizontal layouts and buttons) implement the `Container` trait and expose:
- `.child(impl Into<EasyElement>)` — adds a child
- `.observe(impl IntoObserverSystem)` — attaches a Bevy observer
- `.spawn(&mut Commands)` — finalizes and spawns the tree

### Element hierarchy

Every node that can be inserted into a container implements `Into<EasyElement>`, which makes the typing strict and composition safe:

```rust
EasyVerticalLayout::new()
    .child(EasyText::new("Hello"))              // text
    .child(EasyButton::new().child(...))        // button
    .child(EasyHorizontalLayout::new()...)      // sub-layout
    .spawn(&mut commands);
```

---

## The `EasyNode` trait

Every widget implements `EasyNode`, which exposes a fluent API for Bevy's `Node` **properties**:

| Category | Methods |
|---|---|
| **Size** | `.width()`, `.height()`, `.min_width()`, `.min_height()`, `.max_width()`, `.max_height()` |
| **Position** | `.position()`, `.top()`, `.right()`, `.bottom()`, `.left()` |
| **Alignment** | `.align_items()`, `.justify_items()`, `.align_self()`, `.justify_self()`, `.align_content()`, `.justify_content()` |
| **Spacing** | `.margin(t, r, b, l)`, `.padding(t, r, b, l)`, `.row_gap()`, `.column_gap()` |
| **Borders** | `.border(width, radius)`, `.border_width(t, r, b, l)`, `.border_radius(tl, tr, br, bl)`, `.border_color()` |
| **Flex** | `.flex_direction()`, `.flex_wrap()`, `.flex_grow()`, `.flex_shrink()`, `.flex_basis()` |
| **Misc** | `.display()`, `.box_sizing()`, `.overflow()`, `.aspect_ratio()`, etc. |

> All methods take `self` by value and return `Self`, which makes chaining possible.

---

## Colors (`EasyColor`)

A static helper that exposes a ready-to-use Bevy `Color` palette:

```rust
EasyColor::WHITE
EasyColor::BLACK
EasyColor::RED
EasyColor::DARK_BLUE
EasyColor::LIGHT_GREEN
// etc.

// Or build a custom color:
EasyColor::from_rgba(0.2, 0.4, 0.6, 1.0)
```

---

## Internal Architecture (overview)

```
src/
├── lib.rs                 # Module re-exports
├── prelude.rs             # Ergonomic prelude
├── main.rs                # Application example
├── core/
│   ├── mod.rs
│   ├── container.rs       # `Container` trait (spawn logic)
│   ├── element.rs         # `EasyElement` enum (polymorphic node)
│   └── node.rs            # `EasyNode` trait (`Node` setters)
├── widgets/
│   ├── mod.rs
│   ├── button.rs          # EasyButton / EasyButtonContainer
│   ├── text.rs            # EasyText
│   ├── horizontal_layout.rs
│   └── vertical_layout.rs
└── helpers/
    ├── mod.rs
    └── colors.rs          # `EasyColor` palette
```

### Design principles

1. **Declarative construction without immediate spawn**: `EasyButton::new()` returns an `EasyButtonContainer` (a *builder*), not a bundle.
2. **Separation of bundle / children / observers**: the `Container` stores the Bevy bundle, the `Vec<EasyElement>` and the `Vec<Observer>` independently.
3. **Atomic spawn**: `.spawn(commands)` inserts the bundle, attaches children through `with_children`, then attaches observers. Everything happens in a single recursive call.
4. **Strict typing through `EasyElement`**: it is impossible to insert an invalid widget into a container.

---

## Current Limitations

- No support for `Image`, `ZIndex`, `GlobalZIndex` or `Transform` beyond what `Node` provides.
- Widgets are not (yet) full-fledged Bevy components — they cannot be reused as-is inside a `Query`.
- No automatic focus / keyboard navigation handling.

---

## License

This crate is dual-licensed under either

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option, following the same licensing scheme as [Bevy](https://github.com/bevyengine/bevy).

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
