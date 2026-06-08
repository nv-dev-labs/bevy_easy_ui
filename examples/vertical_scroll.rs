//! Working thanks to event system. Create a scrollable widget for vertical scrolling !
use bevy::prelude::*;
use bevy_easy_ui::prelude::*;

const LINE_HEIGHT: f32 = 21.;

#[derive(EntityEvent, Debug)]
#[entity_event(propagate, auto_propagate)]
struct Scroll {
  entity: Entity,
  delta: Vec2,
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, send_scroll_events)
    .add_observer(on_scroll_handler)
    .run();
}

fn send_scroll_events(
  mut mouse_wheel_reader: MessageReader<bevy::input::mouse::MouseWheel>,
  hover_map: Res<bevy::picking::hover::HoverMap>,
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut commands: Commands,
) {
  use bevy::input::mouse::MouseScrollUnit;
  for mouse_wheel in mouse_wheel_reader.read() {
    let mut delta = -Vec2::new(mouse_wheel.x, mouse_wheel.y);
    if mouse_wheel.unit == MouseScrollUnit::Line {
      delta *= LINE_HEIGHT;
    }
    if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight])
    {
      std::mem::swap(&mut delta.x, &mut delta.y);
    }
    for pointer_map in hover_map.values() {
      for entity in pointer_map.keys().copied() {
        commands.trigger(Scroll { entity, delta });
      }
    }
  }
}

fn on_scroll_handler(
  mut scroll: On<Scroll>,
  mut query: Query<(&mut ScrollPosition, &Node, &ComputedNode)>,
) {
  let Ok((mut scroll_position, node, computed)) = query.get_mut(scroll.entity)
  else {
    return;
  };
  let max_offset = (computed.content_size() - computed.size())
    * computed.inverse_scale_factor();
  let delta = &mut scroll.delta;
  if node.overflow.x == OverflowAxis::Scroll && delta.x != 0. {
    let max = if delta.x > 0. {
      scroll_position.x >= max_offset.x
    } else {
      scroll_position.x <= 0.
    };
    if !max {
      scroll_position.x += delta.x;
      delta.x = 0.;
    }
  }
  if node.overflow.y == OverflowAxis::Scroll && delta.y != 0. {
    let max = if delta.y > 0. {
      scroll_position.y >= max_offset.y
    } else {
      scroll_position.y <= 0.
    };
    if !max {
      scroll_position.y += delta.y;
      delta.y = 0.;
    }
  }
  if *delta == Vec2::ZERO {
    scroll.propagate(false);
  }
}

fn setup(mut commands: Commands) {
  commands.spawn(Camera2d);

  EasyVerticalLayout::new()
    .with_z_index(0)
    .with_background_color(EasyColor::DARK_GRAY)
    .with_width(percent(100.))
    .with_height(percent(100.))
    .with_justify_content(JustifyContent::Center)
    .with_align_items(AlignItems::Center)
    .with_padding(px(20.), px(20.), px(20.), px(20.))
    .with_child(
      EasyVerticalLayout::new()
        .with_z_index(1)
        .with_background_color(EasyColor::BLACK)
        .with_border_color(EasyColor::WHITE)
        .with_border(px(2.), px(8.))
        .with_width(px(320.))
        .with_height(px(300.))
        .with_overflow(Overflow::scroll())
        .with_scrollbar_width(8.0)
        .with_padding(px(10.), px(10.), px(10.), px(10.))
        .with_row_gap(px(6.))
        .with_align_items(AlignItems::Stretch)
        .with_child(
          EasyLabel::new("Scrollable list (try scrolling!)")
            .with_z_index(2)
            .with_color(EasyColor::WHITE)
            .with_font_size(18.),
        )
        .with_child(
          EasyText::new("Item 1")
            .with_color(EasyColor::LIGHT_GRAY)
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 2")
            .with_color(EasyColor::LIGHT_GRAY)
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 3")
            .with_color(EasyColor::LIGHT_GRAY)
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 4")
            .with_color(EasyColor::LIGHT_GRAY)
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 5")
            .with_color(EasyColor::LIGHT_GRAY)
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 6")
            .with_color(EasyColor::LIGHT_GRAY)
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 7")
            .with_color(EasyColor::LIGHT_GRAY)
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 8")
            .with_color(EasyColor::LIGHT_GRAY)
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 9")
            .with_color(EasyColor::LIGHT_GRAY)
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 10")
            .with_color(EasyColor::LIGHT_GRAY)
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 11")
            .with_color(EasyColor::LIGHT_GRAY)
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 12")
            .with_color(EasyColor::LIGHT_GRAY)
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 13")
            .with_color(EasyColor::LIGHT_GRAY)
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 14")
            .with_color(EasyColor::LIGHT_GRAY)
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 15")
            .with_color(EasyColor::LIGHT_GRAY)
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 16")
            .with_color(EasyColor::LIGHT_GRAY)
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 17")
            .with_color(EasyColor::LIGHT_GRAY)
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 18")
            .with_color(EasyColor::LIGHT_GRAY)
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 19")
            .with_color(EasyColor::LIGHT_GRAY)
            .with_z_index(2),
        )
        .with_child(
          EasyText::new("Item 20")
            .with_color(EasyColor::LIGHT_GRAY)
            .with_z_index(2),
        ),
    )
    .spawn(&mut commands);
}
