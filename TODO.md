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
| 2 | `EasyViewport` widget | One of the 5 native Bevy UI widgets is missing | ✅ |
| 3 | `BoxShadow` | Very common, no simple Bevy alternative | ✅ |
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
| `ViewportNode` | ✅ DONE | `EasyViewport` (with `with_target_camera` helper) |

## Core components

| Component | Status | Notes |
|---|---|---|
| `Node` | ✅ DONE | 100% covered via `EasyNode` trait (display, box_sizing, size, position, align, margin, padding, border, flex, grid) |
| `BorderRadius` | ✅ DONE | Via `EasyNode::with_border_radius` |
| `BackgroundColor` | ✅ DONE | Setter on all widgets via `EasyStyleExt` |
| `BorderColor` | ✅ DONE | Setter on all widgets via `EasyStyleExt` |
| `BoxShadow` | ✅ DONE | Setter on all widgets via `EasyStyleExt` |
| `BackgroundGradient` | ❌ TODO | Linear / radial / conic gradients |
| `BorderGradient` | ❌ TODO | Border gradients |
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

All text-bearing widgets (`EasyText`, `EasyLabel`, `EasySpan`, `EasyRichText`) share the same setters via `EasyTextPropsExt`.

| Component | Status | Setter |
|---|---|---|
| `Text` | ✅ DONE | `EasyText::new(text)` |
| `TextLayout` (justify) | ✅ DONE | `.with_justify(Justify::Center)` |
| `TextLayout` (linebreak) | ✅ DONE | `.with_linebreak(LineBreak::WordOrCharacter)` |
| `LineHeight` | ✅ DONE | `.with_line_height(LineHeight::Px(20.0))` |
| `TextSpan` | ✅ DONE | `EasySpan` |
| `TextShadow` (default = invisible) | ✅ DONE | `.with_text_shadow(...)` to enable |

## Image components

| Component | Status | Setter |
|---|---|---|
| `ImageNode.color` (tint) | ✅ DONE | `.with_image_color(Color)` |
| `ImageNode.image` (handle) | ✅ DONE | `.with_image(Handle<Image>)` |
| `ImageNode.flip_x` / `flip_y` | ✅ DONE | `.with_flip_x(bool)` / `.with_flip_y(bool)` |
| `ImageNode.rect` | ✅ DONE | `.with_rect(Rect)` |
| `ImageNode.image_mode` | ✅ DONE | `.with_image_mode(NodeImageMode)` |
| `ImageNode.texture_atlas` | ✅ DONE | `.with_texture_atlas(TextureAtlas)` |

## Viewport widget details

| Component | Status | Setter |
|---|---|---|
| `ViewportNode` | ✅ DONE | `EasyViewport::new(camera)` |
| `EasyViewport` `with_target_camera` | ✅ DONE | `.with_target_camera(Entity)` |
| `EasyViewport` re-exported in `prelude` | ✅ DONE | |
| `EasyViewport` derives `Bundle` | ✅ DONE | |

## Rendering and picking

| Component | Status |
|---|---|
| `UiTargetCamera` | ❌ TODO |
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

## Recently completed (since session start)

- ✅ **`core/parts.rs` created** with reusable `Bundle` parts: `EasyStyle`, `EasyTextProps`, `EasyImageProps`
- ✅ **Bundle auto-flattening used** — Bevy 0.18's `#[derive(Bundle)]` flattens nested bundles without any attribute
- ✅ **Extension traits `EasyStyleExt` / `EasyTextPropsExt`** defined ONCE, used by ALL widgets
- ✅ **All 9 widgets refactored** to use `box_style: EasyStyle` (and `text_style: EasyTextProps` for text widgets)
- ✅ **All setters DRY** — `with_background_color`, `with_border_color`, `with_box_shadow`, `with_color`, `with_font_*`, `with_justify`, `with_linebreak`, `with_line_height`, etc. all defined once
- ✅ **`EasyTextProps::default()`** uses an invisible `TextShadow` (transparent, zero offset) — opt-in via `.with_text_shadow(...)`
- ✅ **`EasyViewport::with_target_camera(Entity)`** helper added
- ✅ **`EasyViewport`** re-exported in `prelude`
- ✅ **`parts::*`** re-exported in `prelude`
- ✅ All comments translated to English
- ✅ License set to MIT OR Apache-2.0 (Bevy-compatible)
- ✅ README translated to English with full API docs

## Remaining known debts

- ❌ **No `Outline` support** — modern focus border missing
- ❌ **No `ZIndex` / `GlobalZIndex` setters** — stacking control missing
- ❌ **No gradients** — `BackgroundGradient`, `BorderGradient`, `LinearGradient`, `RadialGradient` all missing
- ❌ **No scrollable widgets** — `ScrollPosition`, `IgnoreScroll`, dedicated scroll container missing
- ❌ **No focus / keyboard nav** — `Focus`, `TabIndex`, `AutoDirectionalNavigation` missing
- ❌ **No `Checkable` / `Checked` / `InteractionDisabled`** — for checkboxes, radios, disabled state
- ❌ **No `UiTargetCamera` / `RelativeCursorPosition`** — picking helpers missing
- ❌ **No animations / transitions** — Node transition helpers missing
