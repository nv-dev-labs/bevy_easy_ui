# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-05

### Added
- Initial release of `bevy_easy_ui`.
- Builder-pattern abstraction over Bevy 0.18.1 UI: `EasyVerticalLayout`, `EasyHorizontalLayout`, `EasyButton`, `EasyText`, `EasyLabel`, `EasySpan`, `EasyRichText`, `EasyImage`, `EasyViewport`.
- Four extension traits: `EasyNode` (Node properties), `EasyBoxStyleExt` (background/border/shadow), `EasyStackStyleExt` (z-index), `EasyTextStyleExt` (text-specific properties).
- `EasyImageNode` for `ImageNode` properties (color, flip, rect, mode, texture atlas).
- `Container` trait to spawn widgets with children and observers.
- `EasyColor` helper with common colors.
- `examples/` directory with runnable demos: `hello`, `button_with_observers`, `image_button`, `rounded_image`, `scroll_vertical`, `carousel`, `viewport`, `rich_text`.
