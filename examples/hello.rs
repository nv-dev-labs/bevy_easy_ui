use bevy::{prelude::*, ui_widgets::Button};
use bevy_easy_ui::prelude::*;

fn main() {
  App::new()
    .add_plugins((DefaultPlugins, ButtonPlugin))
    .add_systems(Startup, setup)
    .add_systems(Update, update_buttons)
    .run();
}

#[allow(clippy::type_complexity)]
fn update_buttons(
  mut buttons: Query<
    (Has<Pressed>, &Hovered, &mut BackgroundColor),
    (Or<(Changed<Pressed>, Changed<Hovered>)>, With<Button>),
  >,
) {
  for (pressed, hovered, mut background_color) in &mut buttons {
    match (pressed, hovered.0) {
      (true, _) => background_color.0 = RED.into(),
      (false, true) => background_color.0 = BLUE.into(),
      (false, false) => background_color.0 = GRAY.into(),
    }
  }
}

fn setup(mut commands: Commands) {
  commands.spawn(Camera2d);

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
        .with_background_color(DARK_GRAY.into())
        .with_border_color(WHITE.into())
        .with_border(px(2.), px(8.))
        .with_padding_y(px(10.))
        .with_padding_x(px(24.))
        .with_child(
          EasyLabel::new("Hello, Bevy!")
            .with_z_index(2)
            .with_color(WHITE.into())
            .with_font_size(24.),
        ),
    )
    .spawn(&mut commands);
}
