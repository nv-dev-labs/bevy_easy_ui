use bevy::{picking::hover::Hovered, prelude::*, ui_widgets::Button};
use bevy_easy_ui::prelude::*;

fn main() {
  App::new()
    .add_plugins((DefaultPlugins, ButtonPlugin))
    .add_systems(Startup, setup)
    .add_systems(Update, update_buttons)
    .run();
}

fn update_buttons(
  mut buttons: Query<
    (&Hovered, &mut BackgroundColor),
    (Changed<Hovered>, With<Button>),
  >,
) {
  for (hovered, mut background_color) in &mut buttons {
    match hovered.0 {
      true => background_color.0 = RED.into(),
      false => background_color.0 = BLUE.into(),
    }
  }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(Camera2d);

  let icon = asset_server.load("bevy.png");

  EasyVerticalLayout::new()
    .with_z_index(0)
    .with_width(percent(100.))
    .with_height(percent(100.))
    .with_justify_content(JustifyContent::Center)
    .with_align_items(AlignItems::Center)
    .with_row_gap(px(20.))
    .with_child(
      EasyButton::new()
        .with_z_index(1)
        .with_background_color(BLUE.into())
        .with_border_color(BLACK.into())
        .with_border(px(2.), px(50.))
        .with_padding(px(8.))
        .with_child(
          EasyImage::new()
            .with_image(icon)
            .with_z_index(2)
            .with_width(px(64.))
            .with_height(px(64.)),
        ),
    )
    .spawn(&mut commands);
}
