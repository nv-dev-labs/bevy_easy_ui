use bevy::prelude::*;
use bevy_easy_ui::prelude::*;

fn main() {
  App::new()
    .add_plugins((DefaultPlugins, CheckboxPlugin))
    .add_systems(Startup, setup)
    .run();
}

fn update_checkbox(new_value: On<ValueChange<bool>>, mut commands: Commands) {
  let color: Color = if new_value.value {
    GREEN.into()
  } else {
    Color::NONE
  };
  commands
    .entity(new_value.source)
    .insert(BackgroundColor(color));
}

fn setup(mut commands: Commands) {
  commands.spawn(Camera2d);

  EasyHorizontalLayout::new()
    .with_z_index(0)
    .with_width(percent(100.))
    .with_height(percent(100.))
    .with_justify_content(JustifyContent::Center)
    .with_align_items(AlignItems::Center)
    .with_child(
      EasyVerticalLayout::new()
        .with_z_index(1)
        .with_padding(px(2.))
        .with_justify_content(JustifyContent::Center)
        .with_align_items(AlignItems::Center)
        .with_border(px(2.), px(5.))
        .with_border_color(WHITE.into())
        .with_child(
          EasyCheckbox::new()
            .with_z_index(2)
            .with_width(px(8.))
            .with_height(px(8.))
            .with_observer(checkbox_self_update)
            .with_observer(update_checkbox),
        ),
    )
    .with_child(
      EasyLabel::new("Check me!")
        .with_z_index(2)
        .with_margin_left(px(2.))
        .with_color(GOLD.into()),
    )
    .spawn(&mut commands);
}
