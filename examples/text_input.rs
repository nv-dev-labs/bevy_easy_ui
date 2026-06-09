use bevy::prelude::*;
use bevy_easy_ui::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(TextInputPlugin)
    .add_systems(Startup, setup)
    .run();
}

//>--------------------- SETUP ---------------------

fn setup(mut commands: Commands) {
  commands.spawn(Camera2d);

  EasyVerticalLayout::new()
    .with_z_index(0)
    .with_width(px(200.))
    .with_height(px(50.))
    .with_display(Display::Flex)
    .with_align_content(AlignContent::Center)
    .with_padding(px(8.))
    .with_background_color(WHITE.into())
    .with_border(px(3.), px(6.))
    .with_border_color(BLACK.into())
    .with_child(
      EasyTextInput::new()
        .with_z_index(1)
        .with_single_line(true)
        .with_width(percent(100.))
        .with_height(px(50.))
        .with_color(RED.into()),
    )
    .spawn(&mut commands);
}
