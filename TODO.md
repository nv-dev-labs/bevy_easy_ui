# bevy_easy_ui — Bevy UI coverage recap

Inventory of Bevy UI 0.18.1 features and their coverage status in `bevy_easy_ui`.

## Status legend

- ❌ — TODO (missing / not implemented)
- 🔧 — DOING (work in progress)
- ✅ — DONE (implemented)

## Top 10 priority gaps (by impact)

| # | Component / Feature | Why priority | Status |
|---|---|---|---|
| 1 | `BackgroundColor` setters on all containers | Critical — impossible to color a layout background today | ❌ |
| 2 | `EasyViewport` widget | One of the 5 native Bevy UI widgets is missing | ❌ |
| 3 | `BoxShadow` | Very common, no simple Bevy alternative | ❌ |
| 4 | `Outline` | Modern (0.17+), important for accessibility | ❌ |
| 5 | `ZIndex` / `GlobalZIndex` | Critical for stacking elements | ❌ |
| 6 | `TextLayout` (justify, linebreak) | Multi-line text is a very common case | ❌ |
| 7 | `ScrollPosition` / scrollable container | Ultra-common use case (lists, scroll views) | ❌ |
| 8 | Advanced `ImageNode` setters | `EasyImage` is limited without them | ❌ |
| 9 | Gradients (`BackgroundGradient`, `BorderGradient`) | Native in Bevy 0.15+ | ❌ |
| 10 | Focus / `TabIndex` | Basic accessibility | ❌ |

## Native Bevy UI widgets

| Widget Bevy | Covered? | `bevy_easy_ui` equivalent |
|---|---|---|
| `Button` | ✅ DONE | `EasyButton` |
| `Text` | ✅ DONE | `EasyText` |
| `Label` | ✅ DONE | `EasyLabel` |
| `ImageNode` | ✅ DONE | `EasyImage` |
| `ViewportNode` | ❌ TODO | `EasyViewport` (missing) |

## Core components

| Component | Status | Notes |
|---|---|---|
| `Node` | ✅ DONE | 100% covered via `EasyNode` trait (display, box_sizing, size, position, align, margin, padding, border, flex, grid) |
| `BorderRadius` | ✅ DONE | Via `EasyNode::with_border_radius` |
| `BackgroundColor` | 🔧 DOING | Setter only on `EasyButton`, not on layouts/containers |
| `BorderColor` | 🔧 DOING | Setter only on `EasyButton` / `EasyImage`, not on layouts |
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

`EasyText` currently exposes: `with_text`, `with_color`, `with_text_shadow`, `with_font_family`, `with_font_size`, `with_font_weight`, `with_smoothing`, `with_features`.

| Component | Status | Missing setter |
|---|---|---|
| `TextLayout` (justify) | ❌ TODO | `.with_justify(JustifyText::Center)` |
| `TextLayout` (linebreak) | ❌ TODO | `.with_linebreak(LineBreak::WordOrCharacter)` |
| `LineHeight` | ❌ TODO | `.with_line_height(...)` |
| `TextSpan` | ✅ DONE | `EasySpan` |

## Image components

`EasyImage` currently exposes: `with_border_color`, plus the `EasyImageNode` trait.

| Component | Status | Missing setter |
|---|---|---|
| `ImageNode.color` (tint) | ❌ TODO | `.with_color(Color)` |
| `ImageNode.flip_x` / `flip_y` | 🔧 DOING | Via `EasyImageNode` (verify) |
| `ImageNode.rect` | ❌ TODO | `.with_rect(Rect)` |
| `ImageNode.image_mode` | ❌ TODO | `.with_mode(NodeImageMode::Fill)` |
| `ImageNode.texture_atlas` | ❌ TODO | `.with_texture_atlas(TextureAtlas)` |

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
