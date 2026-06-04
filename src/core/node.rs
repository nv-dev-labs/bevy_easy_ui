use bevy::ui::*;

#[allow(dead_code)]
/// A trait for UI elements that have a Node component, allowing for easy configuration of layout and styling properties.
pub trait EasyNode: Sized {
    fn node_mut(&mut self) -> &mut Node;

    fn with_display(mut self, display: Display) -> Self {
        self.node_mut().display = display;
        self
    }

    fn with_box_sizing(mut self, box_sizing: BoxSizing) -> Self {
        self.node_mut().box_sizing = box_sizing;
        self
    }

    fn with_overflow(mut self, overflow: Overflow) -> Self {
        self.node_mut().overflow = overflow;
        self
    }

    fn with_scrollbar_width(mut self, scrollbar_width: f32) -> Self {
        self.node_mut().scrollbar_width = scrollbar_width;
        self
    }

    fn with_overflow_clip_margin(mut self, overflow_clip_margin: OverflowClipMargin) -> Self {
        self.node_mut().overflow_clip_margin = overflow_clip_margin;
        self
    }

    fn with_aspect_ratio(mut self, aspect_ratio: f32) -> Self {
        self.node_mut().aspect_ratio = Some(aspect_ratio);
        self
    }

    //? SIZE PROPERTIES

    fn with_width(mut self, width: Val) -> Self {
        self.node_mut().width = width;
        self
    }

    fn with_height(mut self, height: Val) -> Self {
        self.node_mut().height = height;
        self
    }

    fn with_min_width(mut self, min_width: Val) -> Self {
        self.node_mut().min_width = min_width;
        self
    }

    fn with_min_height(mut self, min_height: Val) -> Self {
        self.node_mut().min_height = min_height;
        self
    }

    fn with_max_width(mut self, max_width: Val) -> Self {
        self.node_mut().max_width = max_width;
        self
    }

    fn with_max_height(mut self, max_height: Val) -> Self {
        self.node_mut().max_height = max_height;
        self
    }

    //? POSITION PROPERTIES

    fn with_position(mut self, position: PositionType) -> Self {
        self.node_mut().position_type = position;
        self
    }

    fn with_right(mut self, right: Val) -> Self {
        self.node_mut().right = right;
        self
    }

    fn with_left(mut self, left: Val) -> Self {
        self.node_mut().left = left;
        self
    }

    fn with_top(mut self, top: Val) -> Self {
        self.node_mut().top = top;
        self
    }

    fn with_bottom(mut self, bottom: Val) -> Self {
        self.node_mut().bottom = bottom;
        self
    }

    //? ALIGNMENT PROPERTIES

    fn with_align_items(mut self, align_items: AlignItems) -> Self {
        self.node_mut().align_items = align_items;
        self
    }

    fn with_justify_items(mut self, justify_items: JustifyItems) -> Self {
        self.node_mut().justify_items = justify_items;
        self
    }

    fn with_align_self(mut self, align_self: AlignSelf) -> Self {
        self.node_mut().align_self = align_self;
        self
    }

    fn with_justify_self(mut self, justify_self: JustifySelf) -> Self {
        self.node_mut().justify_self = justify_self;
        self
    }

    fn with_align_content(mut self, align_content: AlignContent) -> Self {
        self.node_mut().align_content = align_content;
        self
    }

    fn with_justify_content(mut self, justify_content: JustifyContent) -> Self {
        self.node_mut().justify_content = justify_content;
        self
    }

    //? MARGIN AND PADDING

    fn with_margin(mut self, top: Val, right: Val, bottom: Val, left: Val) -> Self {
        self.node_mut().margin = UiRect { top, right, bottom, left };
        self
    }

    fn with_padding(mut self, top: Val, right: Val, bottom: Val, left: Val) -> Self {
        self.node_mut().padding = UiRect { top, right, bottom, left };
        self
    }

    //? BORDER PROPERTIES

    fn with_border(
        mut self,
        border_width: Val,
        border_radius: Val
    ) -> Self {
        self.node_mut().border = UiRect::all(border_width);
        self.node_mut().border_radius = BorderRadius::all(border_radius);
        self
    }

    fn with_border_width(mut self, top: Val, right: Val, bottom: Val, left: Val) -> Self {
        self.node_mut().border = UiRect { top, right, bottom, left };
        self
    }

    fn with_border_radius(mut self, top_left: Val, top_right: Val, bottom_right: Val, bottom_left: Val) -> Self {
        self.node_mut().border_radius = BorderRadius { top_left, top_right, bottom_right, bottom_left };
        self
    }

    //? FLEX PROPERTIES

    fn with_flex_direction(mut self, flex_direction: FlexDirection) -> Self {
        self.node_mut().flex_direction = flex_direction;
        self
    }

    fn with_flex_wrap(mut self, flex_wrap: FlexWrap) -> Self {
        self.node_mut().flex_wrap = flex_wrap;
        self
    }

    fn with_flex_grow(mut self, flex_grow: f32) -> Self {
        self.node_mut().flex_grow = flex_grow;
        self
    }

    fn with_flex_shrink(mut self, flex_shrink: f32) -> Self {
        self.node_mut().flex_shrink = flex_shrink;
        self
    }

    fn with_flex_basis(mut self, flex_basis: Val) -> Self {
        self.node_mut().flex_basis = flex_basis;
        self
    }

    //? GRID PROPERTIES

    fn with_row_gap(mut self, row_gap: Val) -> Self {
        self.node_mut().row_gap = row_gap;
        self
    }

    fn with_column_gap(mut self, column_gap: Val) -> Self {
        self.node_mut().column_gap = column_gap;
        self
    }

    fn with_grid_auto_flow(mut self, grid_auto_flow: GridAutoFlow) -> Self {
        self.node_mut().grid_auto_flow = grid_auto_flow;
        self
    }

    fn with_grid_template_rows(mut self, grid_template_rows: Vec<RepeatedGridTrack>) -> Self {
        self.node_mut().grid_template_rows = grid_template_rows;
        self
    }

    fn with_grid_template_columns(mut self, grid_template_columns: Vec<RepeatedGridTrack>) -> Self {
        self.node_mut().grid_template_columns = grid_template_columns;
        self
    }

    fn with_grid_auto_rows(mut self, grid_auto_rows: Vec<GridTrack>) -> Self {
        self.node_mut().grid_auto_rows = grid_auto_rows;
        self
    }

    fn with_grid_auto_columns(mut self, grid_auto_columns: Vec<GridTrack>) -> Self {
        self.node_mut().grid_auto_columns = grid_auto_columns;
        self
    }

    fn with_grid_row(mut self, grid_row: GridPlacement) -> Self {
        self.node_mut().grid_row = grid_row;
        self
    }

    fn with_grid_column(mut self, grid_column: GridPlacement) -> Self {
        self.node_mut().grid_column = grid_column;
        self
    }
}
