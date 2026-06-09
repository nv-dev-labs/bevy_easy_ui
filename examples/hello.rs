use bevy::prelude::*;
use bevy_easy_ui::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands) {
  commands.spawn(Camera2d);

  EasyVerticalLayout::new()
    .with_style(EasyVerticalLayoutStyle {
      node: Node { ..default() },
      box_style: EasyBoxStyle::default(),
      stack_style: EasyStackStyle::default(),
    })
    .with_z_index(0)
    .with_width(percent(100.))
    .with_height(percent(100.))
    .with_justify_content(JustifyContent::Center)
    .with_align_items(AlignItems::Center)
    .with_child(
      EasyButton::new()
        .with_z_index(1)
        .with_background_color(BLUE.into())
        .with_border_color(WHITE.into())
        .with_border(px(2.), px(10.))
        .with_padding_x(px(40.))
        .with_padding_y(px(20.))
        .with_child(
          EasyLabel::new("Hello, Bevy!")
            .with_z_index(2)
            .with_color(WHITE.into())
            .with_font_size(32.),
        ),
    )
    .spawn(&mut commands);
}
