Hi everyone,

Before anything, I would like to thank the Bevy team for this amazing engine and the welcoming community around it. I've been using Bevy for some time now and I really appreciate how powerful and flexible it is.

I've just published the first release of **`bevy_easy_ui`** (v0.1.0) on crates.io. The goal is to make building UIs in Bevy feel a bit more straightforward: it's a fluent builder-pattern abstraction layer that wraps Bevy's UI components in a more intuitive API, with type-checked setters, seamless chaining, and a clear separation between container and non-container widgets. Hope it helps you build UIs faster!

## About

**Before — the standard Bevy way:**

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
        Label,
    ));
});
```

**After — with `bevy_easy_ui`:**

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

## What's in the box

The crate ships a set of `Easy*` builders, each wrapping the matching Bevy component(s):

| Widget | Bevy base | Kind |
|---|---|---|
| `EasyVerticalLayout` / `EasyHorizontalLayout` | `Node` + `FlexDirection` | container |
| `EasyButton` | `Button` + `Node` | container |
| `EasyRichText` | `Text` + `TextSpan` children | container |
| `EasyViewport` | `Node` + `ViewportNode` | container |
| `EasyLabel` / `EasyText` / `EasySpan` | `Text` + `Node` (+ `Label`) | non-container |
| `EasyImage` | `ImageNode` + `Node` | non-container |
| `EasyTextInput` | re-export of `bevy_ui_text_input` | non-container |

A few things worth pointing out:

- **Reusable styles** with `with_style(EasyXxxStyle)` — a single setter that swaps the whole `Node` + `EasyBoxStyle` + `EasyStackStyle` (+ text style where relevant) at once. Handy for theming.
- **`ScrollPlugin`** — a tiny plugin that turns the mouse wheel into a `Scroll` event consumable by any `Overflow::scroll_*()` node. Hold `Ctrl` while scrolling to scroll horizontally instead.
- **Observers work the way you'd expect** — `.with_observer(...)` on any container, attached as a real Bevy observer (`Pointer<Over>`, `Pointer<Out>`, `Pointer<Click>`, etc.).
- **Colors re-exported** from `bevy::color::palettes::css::*`, so `WHITE`, `BLACK`, `BLUE`, `DARK_GRAY`, `LIGHT_BLUE`, … all flow into `.with_*_color(...into())` without extra imports.

## What it doesn't try to do (yet)

`bevy_easy_ui` is still under active development, and the 0.1.0 release focuses on getting the core layout, text, image, and input surfaces right. Some things are out of scope for now: `Checkbox`, `Slider`, `RadioButton` + `RadioGroup`, and `Scrollbar`.

**0.1.x is young** — the API works and is covered by eight examples, but it's still an early release. Names and patterns may move. Bug reports and PRs are genuinely welcome, and any feedback you have is greatly appreciated.

## Version compatibility

| `bevy_easy_ui` | `bevy_ui_text_input` | `bevy` |
|---|---|---|
| 0.1.0 | 0.7.0 | 0.18.1 |

## How to try it

Add it to your `Cargo.toml`:

```toml
[dependencies]
bevy = "0.18.1"
bevy_easy_ui = "0.1.0"
```

…then:

```bash
cargo run --example hello
cargo run --example scroll
cargo run --example text_input
```

(The repo has eight examples in total — `hello`, `button_with_observers`, `image_button`, `rounded_image`, `scroll`, `viewport`, `rich_text`, and `text_input`.)

## Links

- **crates.io:** https://crates.io/crates/bevy_easy_ui
- **docs.rs:** https://docs.rs/bevy_easy_ui
- **repo:** https://github.com/nv-dev-labs/bevy_easy_ui
- **license:** dual MIT / Apache-2.0
