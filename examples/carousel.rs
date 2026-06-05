use bevy::prelude::*;
use bevy_easy_ui::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(Camera2d);

  let image = asset_server.load("bevy.png");

  let mut carousel = EasyVerticalLayout::new()
    .with_z_index(0)
    .with_background_color(EasyColor::DARK_GRAY)
    .with_width(percent(100.))
    .with_height(percent(100.))
    .with_justify_content(JustifyContent::Center)
    .with_align_items(AlignItems::Center)
    .with_row_gap(px(16.))
    .with_child(
      EasyLabel::new("Horizontal scroll carousel")
        .with_z_index(1)
        .with_color(EasyColor::WHITE)
        .with_font_size(20.),
    );

  let row = EasyHorizontalLayout::new()
    .with_z_index(1)
    .with_background_color(EasyColor::BLACK)
    .with_border_color(EasyColor::WHITE)
    .with_border(px(2.), px(8.))
    .with_width(px(120.))
    .with_height(px(160.))
    .with_overflow(Overflow::scroll_x())
    .with_scrollbar_width(8.0)
    .with_padding(px(10.), px(10.), px(10.), px(10.))
    .with_column_gap(px(10.))
    .with_align_items(AlignItems::Center);

  let mut row = row;
  for i in 0..12 {
    row = row.with_child(
      EasyLabel::new(format!("Item {i}"))
        .with_z_index(2)
        .with_min_width(px(120.))
        .with_background_color(EasyColor::BLUE)
        .with_padding(px(10.), px(10.), px(10.), px(10.))
        .with_flex_shrink(0.0),
    );
  }

  carousel = carousel.with_child(row);
  carousel.spawn(&mut commands);
}
