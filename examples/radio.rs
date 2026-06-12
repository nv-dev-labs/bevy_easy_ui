use bevy::{picking::hover::Hovered, prelude::*};
use bevy_easy_ui::prelude::*;

fn main() {
  App::new()
    .add_plugins((DefaultPlugins, RadioGroupPlugin))
    .add_systems(Startup, setup)
    .add_systems(Update, update_radio_button)
    .add_systems(
      Update,
      update_radio_button_removed.after(update_radio_button),
    )
    .add_observer(on_radio_value_change)
    .run();
}

/// Update the background color of the radio button when the [`Checked`] component is removed.
fn update_radio_button_removed(
  mut radio_buttons: Query<(&Hovered, &mut BackgroundColor), With<RadioButton>>,
  mut removed: RemovedComponents<Checked>,
) {
  for entity in removed.read() {
    if let Ok((hovered, mut bg)) = radio_buttons.get_mut(entity) {
      bg.0 = if hovered.0 { BLUE.into() } else { GRAY.into() };
    }
  }
}

/// Handle the logic of checking the radio button when it is clicked, and unchecking all its siblings in the same radio group.
fn on_radio_value_change(
  ev: On<ValueChange<bool>>,
  q_groups: Query<Entity, With<RadioGroup>>,
  q_parents: Query<&ChildOf>,
  q_children: Query<&Children>,
  q_radios: Query<Entity, (With<RadioButton>, With<Checked>)>,
  mut commands: Commands,
) {
  let Some(group) = q_parents
    .iter_ancestors(ev.source)
    .find(|e| q_groups.contains(*e))
  else {
    return;
  };

  for descendant in q_children.iter_descendants(group) {
    if descendant == ev.source {
      commands.entity(descendant).insert(Checked);
    } else if q_radios.contains(descendant) {
      commands.entity(descendant).remove::<Checked>();
    }
  }
}

/// Update the background color of the radio button based on its state (checked, hovered, or normal).
fn update_radio_button(
  mut radio_buttons: Query<
    (Has<Checked>, &Hovered, &mut BackgroundColor),
    (Or<(Added<Checked>, Changed<Hovered>)>, With<RadioButton>),
  >,
) {
  for (checked, hovered, mut background_color) in &mut radio_buttons {
    match (checked, hovered.0) {
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
    .with_child(
      EasyRadioGroup::new("group1")
        .with_display(Display::Flex)
        .with_flex_direction(FlexDirection::Column)
        .with_z_index(1)
        .with_padding(px(2.))
        .with_justify_content(JustifyContent::Center)
        .with_align_items(AlignItems::Center)
        .with_border(px(2.), px(5.))
        .with_child(
          EasyHorizontalLayout::new()
            .with_z_index(2)
            .with_width(percent(100.))
            .with_height(percent(100.))
            .with_justify_content(JustifyContent::SpaceAround)
            .with_align_items(AlignItems::Center)
            .with_child(
              EasyRadioButton::new()
                .with_z_index(3)
                .with_width(px(24.))
                .with_height(px(24.))
                .with_border(px(1.), px(50.))
                .with_padding(px(2.))
                .with_border_color(WHITE.into()),
            )
            .with_child(
              EasyLabel::new("Option 1")
                .with_z_index(3)
                .with_padding(px(2.))
                .with_margin_left(px(4.)),
            ),
        )
        .with_child(
          EasyHorizontalLayout::new()
            .with_z_index(2)
            .with_width(percent(100.))
            .with_height(percent(100.))
            .with_justify_content(JustifyContent::SpaceAround)
            .with_align_items(AlignItems::Center)
            .with_child(
              EasyRadioButton::new()
                .with_z_index(3)
                .with_width(px(24.))
                .with_height(px(24.))
                .with_border(px(1.), px(50.))
                .with_padding(px(2.))
                .with_border_color(WHITE.into()),
            )
            .with_child(
              EasyLabel::new("Option 2")
                .with_z_index(3)
                .with_padding(px(2.))
                .with_margin_left(px(4.)),
            ),
        ),
    )
    .spawn(&mut commands);
}
