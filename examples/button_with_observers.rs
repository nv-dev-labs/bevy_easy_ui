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
    .insert(BackgroundColor(EasyColor::BLUE));
}

fn hover_out(hover_out: On<Pointer<Out>>, mut commands: Commands) {
  commands
    .entity(hover_out.entity)
    .insert(BackgroundColor(EasyColor::DARK_GRAY));
}

fn click(click: On<Pointer<Click>>, mut commands: Commands) {
  commands
    .entity(click.entity)
    .insert(BackgroundColor(EasyColor::RED));
}

fn setup(mut commands: Commands) {
  commands.spawn(Camera2d);

  EasyVerticalLayout::new()
    .with_z_index(0)
    .with_background_color(EasyColor::BLACK)
    .with_width(percent(100.))
    .with_height(percent(100.))
    .with_justify_content(JustifyContent::Center)
    .with_align_items(AlignItems::Center)
    .with_row_gap(px(20.))
    .with_child(
      EasyButton::new()
        .with_z_index(1)
        .with_background_color(EasyColor::DARK_GRAY)
        .with_border_color(EasyColor::WHITE)
        .with_border(px(2.), px(8.))
        .with_padding(px(10.), px(24.), px(10.), px(24.))
        .with_observer(hover_in)
        .with_observer(hover_out)
        .with_observer(click)
        .with_child(
          EasyLabel::new("Click me!")
            .with_z_index(2)
            .with_color(EasyColor::WHITE)
            .with_font_size(24.),
        ),
    )
    .with_child(
      EasyButton::new()
        .with_z_index(1)
        .with_background_color(EasyColor::DARK_GRAY)
        .with_border_color(EasyColor::WHITE)
        .with_border(px(2.), px(8.))
        .with_padding(px(10.), px(24.), px(10.), px(24.))
        .with_observer(hover_in)
        .with_observer(hover_out)
        .with_observer(click)
        .with_child(
          EasyLabel::new("Submit")
            .with_z_index(2)
            .with_color(EasyColor::WHITE)
            .with_font_size(24.),
        ),
    )
    .with_child(
      EasyButton::new()
        .with_z_index(1)
        .with_background_color(EasyColor::DARK_GRAY)
        .with_border_color(EasyColor::WHITE)
        .with_border(px(2.), px(8.))
        .with_padding(px(10.), px(24.), px(10.), px(24.))
        .with_observer(hover_in)
        .with_observer(hover_out)
        .with_observer(click)
        .with_child(
          EasyLabel::new("Cancel")
            .with_z_index(2)
            .with_color(EasyColor::WHITE)
            .with_font_size(24.),
        ),
    )
    .spawn(&mut commands);
}
