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
    .with_z_index(0)
    .with_width(percent(100.))
    .with_height(percent(100.))
    .with_justify_content(JustifyContent::Center)
    .with_align_items(AlignItems::Center)
    .with_padding(px(40.))
    .with_child(
      EasyRichText::new()
        .with_z_index(1)
        .with_font_size(28.)
        .with_color(WHITE.into())
        .with_child(
          EasySpan::new("Rich text demo. ")
            .with_color(WHITE.into())
            .with_font_size(28.),
        )
        .with_child(
          EasySpan::new("This part is red. ")
            .with_color(RED.into())
            .with_font_size(32.),
        )
        .with_child(
          EasySpan::new("This part is green and italic. ")
            .with_color(GREEN.into())
            .with_justify(Justify::Center)
            .with_font_size(24.),
        )
        .with_child(
          EasySpan::new("And this part is blue with a larger size. ")
            .with_color(LIGHT_BLUE.into())
            .with_font_size(36.),
        )
        .with_child(
          EasySpan::new("Back to white.")
            .with_color(WHITE.into())
            .with_font_size(20.),
        ),
    )
    .spawn(&mut commands);
}
