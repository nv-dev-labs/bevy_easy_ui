# Changelog

## [0.1.0] - 2026-06-05

### Added
- Initial release of `bevy_easy_ui`.
- Builder-pattern abstraction over Bevy 0.18.1 UI: `EasyVerticalLayout`, `EasyHorizontalLayout`, `EasyButton`, `EasyText`, `EasyLabel`, `EasySpan`, `EasyRichText`, `EasyImage`, `EasyViewport`.
- Four extension traits: `EasyNode` (Node properties), `EasyBoxStyleExt` (background/border/shadow), `EasyStackStyleExt` (z-index), `EasyTextStyleExt` (text-specific properties).
- `EasyImageNode` for `ImageNode` properties (color, flip, rect, mode, texture atlas).
- `EasyNode` for `Node` properties (size, position, alignment, spacing, borders, flex, grid, overflow, display).
- `Container` trait to spawn widgets with children and observers.
- `EasyColor` helper with common colors.
- `examples/` directory with runnable demos: `hello`, `button_with_observers`, `image_button`, `rounded_image`, `scroll_vertical`, `carousel`, `viewport`, `rich_text`.
