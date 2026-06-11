use bevy::ui::*;

#[allow(dead_code)]
/// A trait for UI elements that have a Node component, allowing for easy configuration of layout and styling properties.
pub trait EasyNode: Sized {
  fn node_mut(&mut self) -> &mut Node;

  /// See [`Node::display`]
  fn with_display(mut self, display: Display) -> Self {
    self.node_mut().display = display;
    self
  }

  /// See [`Node::box_sizing`]
  fn with_box_sizing(mut self, box_sizing: BoxSizing) -> Self {
    self.node_mut().box_sizing = box_sizing;
    self
  }

  /// See [`Node::overflow`]
  fn with_overflow(mut self, overflow: Overflow) -> Self {
    self.node_mut().overflow = overflow;
    self
  }

  /// See [`Node::scrollbar_width`]
  fn with_scrollbar_width(mut self, scrollbar_width: f32) -> Self {
    self.node_mut().scrollbar_width = scrollbar_width;
    self
  }

  /// See [`Node::overflow_clip_margin`]
  fn with_overflow_clip_margin(
    mut self,
    overflow_clip_margin: OverflowClipMargin,
  ) -> Self {
    self.node_mut().overflow_clip_margin = overflow_clip_margin;
    self
  }

  /// See [`Node::aspect_ratio`]
  fn with_aspect_ratio(mut self, aspect_ratio: f32) -> Self {
    self.node_mut().aspect_ratio = Some(aspect_ratio);
    self
  }

  //? SIZE PROPERTIES

  /// See [`Node::width`]
  fn with_width(mut self, width: Val) -> Self {
    self.node_mut().width = width;
    self
  }

  /// See [`Node::height`]
  fn with_height(mut self, height: Val) -> Self {
    self.node_mut().height = height;
    self
  }

  /// See [`Node::min_width`]
  fn with_min_width(mut self, min_width: Val) -> Self {
    self.node_mut().min_width = min_width;
    self
  }

  /// See [`Node::min_height`]
  fn with_min_height(mut self, min_height: Val) -> Self {
    self.node_mut().min_height = min_height;
    self
  }

  /// See [`Node::max_width`]
  fn with_max_width(mut self, max_width: Val) -> Self {
    self.node_mut().max_width = max_width;
    self
  }

  /// See [`Node::max_height`]
  fn with_max_height(mut self, max_height: Val) -> Self {
    self.node_mut().max_height = max_height;
    self
  }

  //? POSITION PROPERTIES

  /// See [`Node::position_type`]
  fn with_position(mut self, position: PositionType) -> Self {
    self.node_mut().position_type = position;
    self
  }

  /// See [`Node::right`]
  fn with_right(mut self, right: Val) -> Self {
    self.node_mut().right = right;
    self
  }

  /// See [`Node::left`]
  fn with_left(mut self, left: Val) -> Self {
    self.node_mut().left = left;
    self
  }

  /// See [`Node::top`]
  fn with_top(mut self, top: Val) -> Self {
    self.node_mut().top = top;
    self
  }

  /// See [`Node::bottom`]
  fn with_bottom(mut self, bottom: Val) -> Self {
    self.node_mut().bottom = bottom;
    self
  }

  //? ALIGNMENT PROPERTIES

  /// See [`Node::align_items`]
  fn with_align_items(mut self, align_items: AlignItems) -> Self {
    self.node_mut().align_items = align_items;
    self
  }

  /// See [`Node::justify_items`]
  fn with_justify_items(mut self, justify_items: JustifyItems) -> Self {
    self.node_mut().justify_items = justify_items;
    self
  }

  /// See [`Node::align_self`]
  fn with_align_self(mut self, align_self: AlignSelf) -> Self {
    self.node_mut().align_self = align_self;
    self
  }

  /// See [`Node::justify_self`]
  fn with_justify_self(mut self, justify_self: JustifySelf) -> Self {
    self.node_mut().justify_self = justify_self;
    self
  }

  /// See [`Node::align_content`]
  fn with_align_content(mut self, align_content: AlignContent) -> Self {
    self.node_mut().align_content = align_content;
    self
  }

  /// See [`Node::justify_content`]
  fn with_justify_content(mut self, justify_content: JustifyContent) -> Self {
    self.node_mut().justify_content = justify_content;
    self
  }

  //? MARGIN AND PADDING

  /// See [`Node::margin`]
  fn with_margin(mut self, value: Val) -> Self {
    self.node_mut().margin = UiRect::all(value);
    self
  }

  /// See [`Node::margin`]
  fn with_margin_top(mut self, top: Val) -> Self {
    self.node_mut().margin.top = top;
    self
  }

  /// See [`Node::margin`]
  fn with_margin_right(mut self, right: Val) -> Self {
    self.node_mut().margin.right = right;
    self
  }

  /// See [`Node::margin`]
  fn with_margin_bottom(mut self, bottom: Val) -> Self {
    self.node_mut().margin.bottom = bottom;
    self
  }

  /// See [`Node::margin`]
  fn with_margin_left(mut self, left: Val) -> Self {
    self.node_mut().margin.left = left;
    self
  }

  /// See [`Node::margin`]
  fn with_margin_x(mut self, x: Val) -> Self {
    self.node_mut().margin.right = x;
    self.node_mut().margin.left = x;
    self
  }

  /// See [`Node::margin`]
  fn with_margin_y(mut self, y: Val) -> Self {
    self.node_mut().margin.top = y;
    self.node_mut().margin.bottom = y;
    self
  }

  /// See [`Node::padding`]
  fn with_padding(mut self, value: Val) -> Self {
    self.node_mut().padding = UiRect::all(value);
    self
  }

  /// See [`Node::padding`]
  fn with_padding_top(mut self, top: Val) -> Self {
    self.node_mut().padding.top = top;
    self
  }

  /// See [`Node::padding`]
  fn with_padding_right(mut self, right: Val) -> Self {
    self.node_mut().padding.right = right;
    self
  }

  /// See [`Node::padding`]
  fn with_padding_bottom(mut self, bottom: Val) -> Self {
    self.node_mut().padding.bottom = bottom;
    self
  }

  /// See [`Node::padding`]
  fn with_padding_left(mut self, left: Val) -> Self {
    self.node_mut().padding.left = left;
    self
  }

  /// See [`Node::padding`]
  fn with_padding_x(mut self, x: Val) -> Self {
    self.node_mut().padding.right = x;
    self.node_mut().padding.left = x;
    self
  }

  /// See [`Node::padding`]
  fn with_padding_y(mut self, y: Val) -> Self {
    self.node_mut().padding.top = y;
    self.node_mut().padding.bottom = y;
    self
  }

  //? BORDER PROPERTIES

  /// See [`Node::border`] and [`Node::border_radius`]
  fn with_border(mut self, border_width: Val, border_radius: Val) -> Self {
    self.node_mut().border = UiRect::all(border_width);
    self.node_mut().border_radius = BorderRadius::all(border_radius);
    self
  }

  /// See [`Node::border`]
  fn with_border_width(
    mut self,
    top: Val,
    right: Val,
    bottom: Val,
    left: Val,
  ) -> Self {
    self.node_mut().border = UiRect {
      top,
      right,
      bottom,
      left,
    };
    self
  }

  /// See [`Node::border`]
  fn with_border_width_top(mut self, border_width: Val) -> Self {
    self.node_mut().border.top = border_width;
    self
  }

  /// See [`Node::border`]
  fn with_border_width_right(mut self, border_width: Val) -> Self {
    self.node_mut().border.right = border_width;
    self
  }

  /// See [`Node::border`]
  fn with_border_width_bottom(mut self, border_width: Val) -> Self {
    self.node_mut().border.bottom = border_width;
    self
  }

  /// See [`Node::border`]
  fn with_border_width_left(mut self, border_width: Val) -> Self {
    self.node_mut().border.left = border_width;
    self
  }

  /// See [`Node::border_radius`]
  fn with_border_radius(mut self, value: Val) -> Self {
    self.node_mut().border_radius = BorderRadius {
      top_left: value,
      top_right: value,
      bottom_right: value,
      bottom_left: value,
    };
    self
  }

  /// See [`Node::border_radius`]
  fn with_border_radius_top_left(mut self, border_radius: Val) -> Self {
    self.node_mut().border_radius.top_left = border_radius;
    self
  }

  /// See [`Node::border_radius`]
  fn with_border_radius_top_right(mut self, border_radius: Val) -> Self {
    self.node_mut().border_radius.top_right = border_radius;
    self
  }

  /// See [`Node::border_radius`]
  fn with_border_radius_bottom_right(mut self, border_radius: Val) -> Self {
    self.node_mut().border_radius.bottom_right = border_radius;
    self
  }

  /// See [`Node::border_radius`]
  fn with_border_radius_bottom_left(mut self, border_radius: Val) -> Self {
    self.node_mut().border_radius.bottom_left = border_radius;
    self
  }

  //? FLEX PROPERTIES

  /// See [`Node::flex_direction`]
  fn with_flex_direction(mut self, flex_direction: FlexDirection) -> Self {
    self.node_mut().flex_direction = flex_direction;
    self
  }

  /// See [`Node::flex_wrap`]
  fn with_flex_wrap(mut self, flex_wrap: FlexWrap) -> Self {
    self.node_mut().flex_wrap = flex_wrap;
    self
  }

  /// See [`Node::flex_grow`]
  fn with_flex_grow(mut self, flex_grow: f32) -> Self {
    self.node_mut().flex_grow = flex_grow;
    self
  }

  /// See [`Node::flex_shrink`]
  fn with_flex_shrink(mut self, flex_shrink: f32) -> Self {
    self.node_mut().flex_shrink = flex_shrink;
    self
  }

  /// See [`Node::flex_basis`]
  fn with_flex_basis(mut self, flex_basis: Val) -> Self {
    self.node_mut().flex_basis = flex_basis;
    self
  }

  //? GRID PROPERTIES

  /// See [`Node::row_gap`]
  fn with_row_gap(mut self, row_gap: Val) -> Self {
    self.node_mut().row_gap = row_gap;
    self
  }

  /// See [`Node::column_gap`]
  fn with_column_gap(mut self, column_gap: Val) -> Self {
    self.node_mut().column_gap = column_gap;
    self
  }

  /// See [`Node::grid_auto_flow`]
  fn with_grid_auto_flow(mut self, grid_auto_flow: GridAutoFlow) -> Self {
    self.node_mut().grid_auto_flow = grid_auto_flow;
    self
  }

  /// See [`Node::grid_template_rows`]
  fn with_grid_template_rows(
    mut self,
    grid_template_rows: Vec<RepeatedGridTrack>,
  ) -> Self {
    self.node_mut().grid_template_rows = grid_template_rows;
    self
  }

  /// See [`Node::grid_template_columns`]
  fn with_grid_template_columns(
    mut self,
    grid_template_columns: Vec<RepeatedGridTrack>,
  ) -> Self {
    self.node_mut().grid_template_columns = grid_template_columns;
    self
  }

  /// See [`Node::grid_auto_rows`]
  fn with_grid_auto_rows(mut self, grid_auto_rows: Vec<GridTrack>) -> Self {
    self.node_mut().grid_auto_rows = grid_auto_rows;
    self
  }

  /// See [`Node::grid_auto_columns`]
  fn with_grid_auto_columns(
    mut self,
    grid_auto_columns: Vec<GridTrack>,
  ) -> Self {
    self.node_mut().grid_auto_columns = grid_auto_columns;
    self
  }

  /// See [`Node::grid_row`]
  fn with_grid_row(mut self, grid_row: GridPlacement) -> Self {
    self.node_mut().grid_row = grid_row;
    self
  }

  /// See [`Node::grid_column`]
  fn with_grid_column(mut self, grid_column: GridPlacement) -> Self {
    self.node_mut().grid_column = grid_column;
    self
  }
}
