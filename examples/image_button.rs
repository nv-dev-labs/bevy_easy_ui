use bevy::prelude::*;
use bevy_easy_ui::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .run();
}

fn hover_in(hover_in: On<Pointer<Over>>, mut commands: Commands) {
  commands
    .entity(hover_in.entity)
    .insert(BackgroundColor(EasyColor::LIGHT_GRAY));
}

fn hover_out(hover_out: On<Pointer<Out>>, mut commands: Commands) {
  commands
    .entity(hover_out.entity)
    .insert(BackgroundColor(EasyColor::WHITE));
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(Camera2d);

  let icon = asset_server.load("bevy.png");

  EasyVerticalLayout::new()
    .with_z_index(0)
    .with_background_color(EasyColor::DARK_GRAY)
    .with_width(percent(100.))
    .with_height(percent(100.))
    .with_justify_content(JustifyContent::Center)
    .with_align_items(AlignItems::Center)
    .with_row_gap(px(20.))
    .with_child(
      EasyButton::new()
        .with_z_index(1)
        .with_background_color(EasyColor::BLUE)
        .with_border_color(EasyColor::DARK_BLUE)
        .with_border(px(2.), px(50.))
        .with_padding(px(8.), px(8.), px(8.), px(8.))
        .with_observer(hover_in)
        .with_observer(hover_out)
        .with_child(
          EasyImage::new(icon.clone())
            .with_z_index(2)
            .with_width(px(64.))
            .with_height(px(64.)),
        ),
    )
    .with_child(
      EasyButton::new()
        .with_z_index(1)
        .with_background_color(EasyColor::BLUE)
        .with_border_color(EasyColor::DARK_BLUE)
        .with_border(px(2.), px(50.))
        .with_padding(px(8.), px(8.), px(8.), px(8.))
        .with_observer(hover_in)
        .with_observer(hover_out)
        .with_child(
          EasyImage::new(icon)
            .with_z_index(2)
            .with_width(px(32.))
            .with_height(px(32.)),
        ),
    )
    .spawn(&mut commands);
}
