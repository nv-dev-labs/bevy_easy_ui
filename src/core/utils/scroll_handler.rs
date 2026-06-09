use bevy::prelude::*;
#[derive(EntityEvent, Debug)]
#[entity_event(propagate, auto_propagate)]
pub struct Scroll {
  entity: Entity,
  delta: Vec2,
}

pub struct ScrollPlugin;

impl Plugin for ScrollPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, scroll_events)
      .add_observer(on_scroll_handler);
  }
}

const LINE_HEIGHT: f32 = 21.;

pub fn scroll_events(
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

pub fn on_scroll_handler(
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
