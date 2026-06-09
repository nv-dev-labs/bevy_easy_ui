use bevy::camera::RenderTarget;
use bevy::prelude::*;
use bevy::render::render_resource::{
  Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy_easy_ui::prelude::*;

const VIEWPORT_W: u32 = 640;
const VIEWPORT_H: u32 = 360;

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
  mut images: ResMut<Assets<Image>>,
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

  // 1. Allocate a texture for the 3D camera to render into.
  let image_handle = images.add(make_viewport_image());

  // 2. Spawn the 3D camera with `RenderTarget::Image` so it doesn't
  //    paint the window directly.
  let camera_entity = commands
    .spawn((
      Camera3d::default(),
      Transform::from_translation(Vec3::new(2.5, 2.0, 3.0))
        .looking_at(Vec3::ZERO, Vec3::Y),
      RenderTarget::from(image_handle),
    ))
    .id();
  commands.insert_resource(WorldCamera(camera_entity));

  // 3. Spawn a default 2D camera that paints the window. It owns the
  //    whole UI tree (including the viewport node below).
  commands.spawn(Camera2d);

  // 4. Add `EasyViewport` somewhere in the UI. The `camera` you pass
  //    is the **3D camera** to display, not the 2D one.
  EasyViewport::new(camera_entity)
    .with_position(PositionType::Absolute)
    .with_left(px(20.))
    .with_top(px(20.))
    .with_width(px(VIEWPORT_W as f32))
    .with_height(px(VIEWPORT_H as f32))
    .with_border_color(WHITE.into())
    .with_border(px(2.), px(4.))
    .spawn(&mut commands);

  EasyLabel::new("Top-left: 3D scene rendered into a UI node")
    .with_position(PositionType::Absolute)
    .with_left(px(20.))
    .with_top(px(VIEWPORT_H as f32 + 30.))
    .with_color(WHITE.into())
    .with_font_size(16.)
    .spawn(&mut commands);
}

/// Builds an `Image` of the right size + format for use as a 3D
/// camera's `RenderTarget::Image`. `TEXTURE_BINDING` + `RENDER_ATTACHMENT`
/// usage flags are required so it can be sampled by `ViewportNode` and
/// written by the 3D camera at the same time.
fn make_viewport_image() -> Image {
  let size = Extent3d {
    width: VIEWPORT_W,
    height: VIEWPORT_H,
    depth_or_array_layers: 1,
  };
  let mut image = Image {
    texture_descriptor: TextureDescriptor {
      label: Some("viewport_image"),
      size,
      mip_level_count: 1,
      sample_count: 1,
      dimension: TextureDimension::D2,
      format: TextureFormat::Rgba8UnormSrgb,
      usage: TextureUsages::TEXTURE_BINDING
        | TextureUsages::COPY_DST
        | TextureUsages::RENDER_ATTACHMENT,
      view_formats: &[],
    },
    ..default()
  };
  image.resize(size);
  image
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
