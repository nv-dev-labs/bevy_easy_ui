use bevy::prelude::*;
use bevy_easy_ui::prelude::*;

fn main() {
  App::new()
    .add_plugins((DefaultPlugins, ScrollPlugin))
    .add_systems(Startup, setup)
    .run();
}

const VERTICAL_SCROLL: bool = false;

fn setup(mut commands: Commands) {
  commands.spawn(Camera2d);

  //> Mouse wheel only
  let vertical_scroll = EasyVerticalLayout::new()
    .with_z_index(0)
    .with_background_color(DARK_GRAY.into())
    .with_width(percent(100.))
    .with_height(percent(100.))
    .with_justify_content(JustifyContent::Center)
    .with_align_items(AlignItems::Center)
    .with_padding(px(20.))
    .with_child(
      EasyVerticalLayout::new()
        .with_z_index(1)
        .with_background_color(BLACK.into())
        .with_border_color(WHITE.into())
        .with_border(px(2.), px(8.))
        .with_width(px(320.))
        .with_height(px(300.))
        .with_overflow(Overflow::scroll_y())
        .with_observer(on_scroll_handler)
        .with_scrollbar_width(8.0)
        .with_padding(px(10.))
        .with_row_gap(px(6.))
        .with_align_items(AlignItems::Stretch)
        .with_child(
          EasyLabel::new("Scrollable list")
            .with_z_index(2)
            .with_color(WHITE.into())
            .with_font_size(18.),
        )
        .with_child(
          EasyText::new("Item 1")
            .with_color(LIGHT_GRAY.into())
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 2")
            .with_color(LIGHT_GRAY.into())
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 3")
            .with_color(LIGHT_GRAY.into())
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 4")
            .with_color(LIGHT_GRAY.into())
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 5")
            .with_color(LIGHT_GRAY.into())
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 6")
            .with_color(LIGHT_GRAY.into())
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 7")
            .with_color(LIGHT_GRAY.into())
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 8")
            .with_color(LIGHT_GRAY.into())
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 9")
            .with_color(LIGHT_GRAY.into())
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 10")
            .with_color(LIGHT_GRAY.into())
            .with_z_index(2),
        ),
    );

  //> CTRL + Mouse wheel
  let horizontal_scroll = EasyVerticalLayout::new()
    .with_z_index(0)
    .with_width(percent(100.))
    .with_height(percent(100.))
    .with_justify_content(JustifyContent::Center)
    .with_align_items(AlignItems::Center)
    .with_row_gap(px(16.))
    .with_child(
      EasyHorizontalLayout::new()
        .with_z_index(1)
        .with_background_color(BLACK.into())
        .with_border_color(WHITE.into())
        .with_border(px(2.), px(8.))
        .with_width(px(120.))
        .with_height(px(160.))
        .with_overflow(Overflow::scroll_x())
        .with_padding(px(10.))
        .with_column_gap(px(10.))
        .with_align_items(AlignItems::Center)
        .with_child(
          EasyLabel::new("Item 1")
            .with_z_index(2)
            .with_min_width(px(120.))
            .with_background_color(BLUE.into())
            .with_padding(px(10.))
            .with_flex_shrink(0.0),
        )
        .with_child(
          EasyLabel::new("Item 2")
            .with_z_index(2)
            .with_min_width(px(120.))
            .with_background_color(BLUE.into())
            .with_padding(px(10.))
            .with_flex_shrink(0.0),
        )
        .with_child(
          EasyLabel::new("Item 3")
            .with_z_index(2)
            .with_min_width(px(120.))
            .with_background_color(BLUE.into())
            .with_padding(px(10.))
            .with_flex_shrink(0.0),
        )
        .with_child(
          EasyLabel::new("Item 4")
            .with_z_index(2)
            .with_min_width(px(120.))
            .with_background_color(BLUE.into())
            .with_padding(px(10.))
            .with_flex_shrink(0.0),
        )
        .with_child(
          EasyLabel::new("Item 5")
            .with_z_index(2)
            .with_min_width(px(120.))
            .with_background_color(BLUE.into())
            .with_padding(px(10.))
            .with_flex_shrink(0.0),
        ),
    );

  if VERTICAL_SCROLL {
    vertical_scroll.spawn(&mut commands);
  } else {
    horizontal_scroll.spawn(&mut commands);
  }
}
