use bevy::prelude::*;
use bevy_easy_ui::prelude::*;

const TRACK_WIDTH: f32 = 200.;
const TRACK_HEIGHT: f32 = 5.;
const THUMB_WIDTH: f32 = 20.;
const THUMB_HEIGHT: f32 = 20.;
const SLIDER_MIN: f32 = 0.;
const SLIDER_MAX: f32 = 100.;

#[derive(Component)]
struct ExampleSlider;

fn main() {
  App::new()
    .add_plugins((DefaultPlugins, SliderPlugin))
    .add_systems(Startup, setup)
    .add_systems(Update, sync_thumb_position)
    .add_systems(Update, sync_slider_label::<ExampleSlider>)
    .add_observer(slider_self_update)
    .run();
}

fn sync_thumb_position(
  sliders: Query<(&SliderValue, &SliderRange, &Children), With<Slider>>,
  mut thumbs: Query<&mut Node, With<SliderThumb>>,
) {
  for (value, range, children) in &sliders {
    let pct = range.thumb_position(value.0);
    let left_px = pct * (TRACK_WIDTH - THUMB_WIDTH);

    for child in children {
      if let Ok(mut node) = thumbs.get_mut(*child) {
        node.left = Val::Px(left_px);
      }
    }
  }
}

fn sync_slider_label<T: Component>(
  sliders: Query<&SliderValue, With<T>>,
  mut labels: Query<&mut Text, With<T>>,
) {
  let Ok(value) = sliders.single() else {
    return;
  };

  for mut text in &mut labels {
    **text = format!("{:.0}", value.0);
  }
}

fn setup(mut commands: Commands) {
  commands.spawn(Camera2d);

  let layout_entity = EasyVerticalLayout::new()
    .with_z_index(0)
    .with_width(percent(100.))
    .with_height(percent(100.))
    .with_justify_content(JustifyContent::Center)
    .with_align_items(AlignItems::Center)
    .spawn(&mut commands);

  let slider_entity = EasySlider::new()
    .with_z_index(1)
    .with_step(50.)
    .with_width(px(TRACK_WIDTH))
    .with_height(px(TRACK_HEIGHT))
    .with_background_color(BLACK.into())
    .with_border_radius(px(999.))
    .with_range(SLIDER_MIN, SLIDER_MAX)
    .with_child(
      EasySliderThumb::new()
        .with_z_index(2)
        .with_position(PositionType::Absolute)
        .with_width(px(THUMB_WIDTH))
        .with_height(px(THUMB_HEIGHT))
        .with_top(px(-(THUMB_HEIGHT - TRACK_HEIGHT) * 0.5))
        .with_background_color(RED.into())
        .with_border(px(1.), px(999.))
        .with_border_color(WHITE.into()),
    )
    .spawn(&mut commands);

  let label_entity = EasyLabel::new("0")
    .with_z_index(1)
    .with_color(WHITE.into())
    .with_font_size(24.)
    .with_smoothing(FontSmoothing::AntiAliased)
    .spawn(&mut commands);

  // Spawning the slider and its label to get the entities then inserting manually a tag to both of them
  commands.entity(slider_entity).insert(ExampleSlider);
  commands.entity(label_entity).insert(ExampleSlider);

  // Adding the slider and its label as children of the layout
  commands
    .entity(layout_entity)
    .add_children(&[slider_entity, label_entity]);
}
