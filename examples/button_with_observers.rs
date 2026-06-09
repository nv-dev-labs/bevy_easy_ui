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
    .insert(BackgroundColor(BLUE.into()));
}

fn hover_out(hover_out: On<Pointer<Out>>, mut commands: Commands) {
  commands
    .entity(hover_out.entity)
    .insert(BackgroundColor(DARK_GRAY.into()));
}

fn click(click: On<Pointer<Click>>, mut commands: Commands) {
  commands
    .entity(click.entity)
    .insert(BackgroundColor(RED.into()));
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
        .with_observer(hover_in)
        .with_observer(hover_out)
        .with_observer(click)
        .with_child(
          EasyLabel::new("Click me!")
            .with_z_index(2)
            .with_color(WHITE.into())
            .with_font_size(24.),
        ),
    )
    .with_child(
      EasyButton::new()
        .with_z_index(1)
        .with_background_color(DARK_GRAY.into())
        .with_border_color(WHITE.into())
        .with_border(px(2.), px(8.))
        .with_padding_y(px(10.))
        .with_padding_x(px(24.))
        .with_observer(hover_in)
        .with_observer(hover_out)
        .with_observer(click)
        .with_child(
          EasyLabel::new("Submit")
            .with_z_index(2)
            .with_color(WHITE.into())
            .with_font_size(24.),
        ),
    )
    .with_child(
      EasyButton::new()
        .with_z_index(1)
        .with_background_color(DARK_GRAY.into())
        .with_border_color(WHITE.into())
        .with_border(px(2.), px(8.))
        .with_padding_y(px(10.))
        .with_padding_x(px(24.))
        .with_observer(hover_in)
        .with_observer(hover_out)
        .with_observer(click)
        .with_child(
          EasyLabel::new("Cancel")
            .with_z_index(2)
            .with_color(WHITE.into())
            .with_font_size(24.),
        ),
    )
    .spawn(&mut commands);
}
