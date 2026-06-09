use bevy::prelude::*;
use bevy_easy_ui::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, rotate_camera)
    .run();
}

#[derive(Resource)]
struct WorldCamera(Entity);

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands.spawn((
    DirectionalLight::default(),
    Transform::from_translation(Vec3::new(1.0, 2.0, 0.5)),
  ));

  commands.spawn((
    Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    MeshMaterial3d(materials.add(StandardMaterial {
      base_color: Color::srgb(0.8, 0.3, 0.2),
      ..default()
    })),
    Transform::from_xyz(0.0, 0.0, 0.0),
  ));

  let camera_entity = commands
    .spawn((
      Camera3d::default(),
      Transform::from_translation(Vec3::new(2.5, 2.0, 3.0))
        .looking_at(Vec3::ZERO, Vec3::Y),
    ))
    .id();
  commands.insert_resource(WorldCamera(camera_entity));

  commands
    .spawn(Camera2d)
    .insert(UiTargetCamera(camera_entity));

  EasyViewport::new(camera_entity)
    .with_position(PositionType::Absolute)
    .with_left(px(0.))
    .with_top(px(0.))
    .with_width(px(640.))
    .with_height(px(360.))
    .with_border_color(WHITE.into())
    .with_border(px(2.), px(4.))
    .spawn(&mut commands);

  EasyVerticalLayout::new()
    .with_position(PositionType::Absolute)
    .with_right(px(20.))
    .with_top(px(20.))
    .with_padding(px(10.))
    .with_background_color(DARK_GRAY.into())
    .with_child(
      EasyLabel::new(
        "Top-left: ViewportNode rendering a 3D scene into a UI node.",
      )
      .with_color(WHITE.into())
      .with_font_size(16.),
    )
    .spawn(&mut commands);
}

fn rotate_camera(
  time: Res<Time>,
  world_camera: Res<WorldCamera>,
  mut transforms: Query<&mut Transform>,
) {
  let Ok(mut transform) = transforms.get_mut(world_camera.0) else {
    return;
  };
  let t = time.elapsed_secs() * 0.5;
  transform.translation = Vec3::new(3.0 * t.cos(), 2.0, 3.0 * t.sin());
  transform.look_at(Vec3::ZERO, Vec3::Y);
}
