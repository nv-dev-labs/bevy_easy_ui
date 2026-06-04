# bevy_easy_ui — Bevy UI coverage recap

Inventory of Bevy UI 0.18.1 features and their coverage status in `bevy_easy_ui`.

## Status legend

- ❌ — TODO (missing / not implemented)
- 🔧 — DOING (work in progress)
- ✅ — DONE (implemented)

## Top 10 priority gaps (by impact)

| # | Component / Feature | Why priority | Status |
|---|---|---|---|
| 1 | `BackgroundColor` setters on all containers | Critical — impossible to color a layout background today | ✅ |
| 2 | `EasyViewport` widget | One of the 5 native Bevy UI widgets is missing | 🔧 |
| 3 | `BoxShadow` | Very common, no simple Bevy alternative | ❌ |
| 4 | `Outline` | Modern (0.17+), important for accessibility | ❌ |
| 5 | `ZIndex` / `GlobalZIndex` | Critical for stacking elements | ❌ |
| 6 | `TextLayout` (justify, linebreak) | Multi-line text is a very common case | ✅ |
| 7 | `ScrollPosition` / scrollable container | Ultra-common use case (lists, scroll views) | ❌ |
| 8 | Advanced `ImageNode` setters | `EasyImage` is limited without them | ✅ |
| 9 | Gradients (`BackgroundGradient`, `BorderGradient`) | Native in Bevy 0.15+ | ❌ |
| 10 | Focus / `TabIndex` | Basic accessibility | ❌ |

## Native Bevy UI widgets

| Widget Bevy | Covered? | `bevy_easy_ui` equivalent |
|---|---|---|
| `Button` | ✅ DONE | `EasyButton` |
| `Text` | ✅ DONE | `EasyText` |
| `Label` | ✅ DONE | `EasyLabel` |
| `ImageNode` | ✅ DONE | `EasyImage` + `EasyImageNode` trait |
| `ViewportNode` | 🔧 DOING | `EasyViewport` (struct exists, no `Bundle` derive) |

## Core components

| Component | Status | Notes |
|---|---|---|
| `Node` | ✅ DONE | 100% covered via `EasyNode` trait (display, box_sizing, size, position, align, margin, padding, border, flex, grid) |
| `BorderRadius` | ✅ DONE | Via `EasyNode::with_border_radius` |
| `BackgroundColor` | ✅ DONE | Setter on `EasyButton`, `EasyHorizontalLayout`, `EasyVerticalLayout`, `EasyText`, `EasyRichText`, `EasySpan`, `EasyLabel`. Missing on `EasyImage` and `EasyViewport`. |
| `BorderColor` | ✅ DONE | Setter on all bundles that include it (except `EasyViewport` which has no border) |
| `BackgroundGradient` | ❌ TODO | Linear / radial / conic gradients |
| `BorderGradient` | ❌ TODO | Border gradients |
| `BoxShadow` | ❌ TODO | No shadow helper |
| `Outline` | ❌ TODO | Modern focus border (0.17+) |
| `OverflowClipMargin` | ✅ DONE | |
| `Overflow` | ✅ DONE | |
| `ZIndex` | ❌ TODO | |
| `GlobalZIndex` | ❌ TODO | |
| `UiTransform` / `UiGlobalTransform` | ❌ TODO | No translate / rotate / scale in runtime |
| `UiScale` | ❌ TODO | No global UI scale helper |
| `ScrollPosition` | ❌ TODO | No scrollable widget |
| `IgnoreScroll` | ❌ TODO | No scroll propagation control |
| `ContentSize` | ❌ TODO | No custom measure helper |
| `NodeMeasure` / `Measure` | ❌ TODO | No custom measure API |

## Interaction and focus

| Component | Status | Notes |
|---|---|---|
| `Button` | ✅ DONE | `EasyButton` |
| `Interaction` (auto-managed) | ✅ DONE | Indirectly via `.with_observer()` |
| `Checkable` / `Checked` | ❌ TODO | Checkboxes / radios |
| `Pressed` | ❌ TODO | |
| `InteractionDisabled` | ❌ TODO | No `with_disabled(true)` helper |
| `Focus` / auto-focus | ❌ TODO | No `with_auto_focus()` |
| `TabIndex` / `TabGroup` | ❌ TODO | No keyboard navigation |
| `AutoDirectionalNavigation` | ❌ TODO | |

## Text components

`EasyText` / `EasyLabel` / `EasySpan` / `EasyRichText` currently expose: `with_text`, `with_color`, `with_text_shadow`, `with_font_family`, `with_font_size`, `with_font_weight`, `with_smoothing`, `with_features`, `with_background_color`, `with_border_color`, `with_text_layout`, `with_justify`, `with_linebreak`, `with_line_height`, `with_style`.

| Component | Status | Missing setter |
|---|---|---|
| `TextLayout` (justify) | ✅ DONE | `.with_justify(Justify::Center)` |
| `TextLayout` (linebreak) | ✅ DONE | `.with_linebreak(LineBreak::WordOrCharacter)` |
| `LineHeight` | ✅ DONE | `.with_line_height(LineHeight::Px(20.0))` |
| `TextSpan` | ✅ DONE | `EasySpan` |

## Image components

`EasyImage` + `EasyImageNode` trait currently expose all major setters.

| Component | Status | Setter |
|---|---|---|
| `ImageNode.color` (tint) | ✅ DONE | `.with_image_color(Color)` |
| `ImageNode.image` (handle) | ✅ DONE | `.with_image(Handle<Image>)` |
| `ImageNode.flip_x` / `flip_y` | ✅ DONE | `.with_flip_x(bool)` / `.with_flip_y(bool)` |
| `ImageNode.rect` | ✅ DONE | `.with_rect(Rect)` |
| `ImageNode.image_mode` | ✅ DONE | `.with_image_mode(NodeImageMode)` |
| `ImageNode.texture_atlas` | ✅ DONE | `.with_texture_atlas(TextureAtlas)` |
| `EasyImage` `with_background_color` | ❌ TODO | Bundle has no `BackgroundColor` field |

## Rendering and picking

| Component | Status |
|---|---|
| `UiTargetCamera` | ❌ TODO — `.with_target_camera(entity)` |
| `UiPickingCamera` / `UiPickingSettings` | ❌ TODO |
| `RelativeCursorPosition` | ❌ TODO |
| `ComputedNode` | (internal Bevy, no helper needed) |
| `CalculatedClip` | (internal Bevy) |
| `ResolvedBorderRadius` | (internal Bevy) |
| `UiStack` | (internal Bevy) |

## Colors and gradients

| Component | Status |
|---|---|
| `Color` (basic) | ✅ DONE — `EasyColor` |
| `LinearGradient` | ❌ TODO |
| `RadialGradient` | ❌ TODO |
| `ConicGradient` | ❌ TODO |
| `ColorStop` / `AngularColorStop` | ❌ TODO |
| `Gradient` | ❌ TODO |
| `InterleavedGradient` | ❌ TODO |

## Geometry and layout

| Component | Status |
|---|---|
| `Val`, `Val2` | ✅ DONE |
| `UiRect` | ✅ DONE |
| `GridTrack`, `RepeatedGridTrack`, `GridPlacement` | ✅ DONE |
| `Overflow` | ✅ DONE |
| `BoxSizing` | ✅ DONE |

## Animations and transitions

| Component | Status |
|---|---|
| Node transitions / easings | ❌ TODO — no animation helper |

## Viewport widget details

`EasyViewport` is currently a plain struct (not a `Bundle`). It is converted to a tuple `(Node, ViewportNode)` at spawn time. To complete it:

- [ ] Add `#[derive(Bundle)]` on `EasyViewport`
- [ ] Make it a `Container` (optional, for HUD overlay children)
- [ ] Add `with_target_camera(Entity)` helper
- [ ] Re-export in `prelude`

## Recently completed (since session start)

- ✅ **Refactored `Container` trait to be generic** over the child type (`Container<C: Into<EasyElement>>`)
- ✅ **Compile-time type-checked `with_child`** — `EasyRichTextContainer` now refuses any non-`EasySpan` at compile time
- ✅ **Bundles converted from tuple-struct to named-struct** for self-documenting access (`self.node` instead of `self.1`)
- ✅ **`BackgroundColor` and `BorderColor` added** to `EasyText`, `EasyRichText`, `EasyLabel`, `EasySpan`, `EasyVerticalLayout`, `EasyHorizontalLayout`
- ✅ **Setters `with_background_color` / `with_border_color` added** on all bundles that include them
- ✅ **`with_text_layout(TextLayout)` setter added** on text widgets
- ✅ **`with_justify(Justify)` sugar added** on text widgets
- ✅ **`with_linebreak(LineBreak)` sugar added** on text widgets
- ✅ **`LineHeight` field and `with_line_height` setter added** on text widgets
- ✅ **`EasyViewport` widget skeleton created**
- ✅ **`EasyImageNode` trait** exposing all `ImageNode` setters (`with_image_color`, `with_image`, `with_texture_atlas`, `with_flip_x`, `with_flip_y`, `with_rect`, `with_image_mode`)
- ✅ **All comments translated to English**
- ✅ **License set to MIT OR Apache-2.0** (Bevy-compatible), `LICENSE-MIT` and `LICENSE-APACHE` files added, `Cargo.toml` updated
- ✅ **README translated to English** with full API docs

## Known design debts

- 🔧 **Massive setter duplication** — 8 widgets × ~10 setters each. Solution being designed: extension traits + blanket impl, or `impl_widget!` macro.
- 🔧 **`with_background_color` on `EasyImage`** — `BackgroundColor` missing from its bundle
- 🔧 **`EasyViewport` not derived as `Bundle`** — should be added for consistency
