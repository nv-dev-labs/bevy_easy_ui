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

  let image = asset_server.load("200.png");

  EasyHorizontalLayout::new()
    .with_z_index(0)
    .with_width(percent(100.))
    .with_height(percent(100.))
    .with_justify_content(JustifyContent::Center)
    .with_align_items(AlignItems::Center)
    .with_column_gap(px(20.))
    .with_padding(px(20.))
    .with_child(
      EasyImage::new()
        .with_image(image.clone())
        .with_width(px(100.))
        .with_height(px(100.))
    )
    .with_child(
      EasyImage::new()
        .with_image(image.clone())
        .with_width(px(100.))
        .with_height(px(100.))
        .with_border_radius(px(10.)),
    )
    .with_child(
      EasyImage::new()
        .with_image(image.clone())
        .with_width(px(100.))
        .with_height(px(100.))
        .with_border_radius(px(50.)),
    )
    .with_child(
      EasyImage::new()
        .with_image(image)
        .with_width(px(100.))
        .with_height(px(100.))
        .with_border_radius(px(50.))
        .with_border_radius_top_right(px(10.))
        .with_border_radius_bottom_left(px(10.)),
    )
    .spawn(&mut commands);
}
