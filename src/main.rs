use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

struct HelloPlugin;

#[derive(Component)]
struct HexTile;

#[derive(Component)]
pub struct AxialCoordinate{q: i32, r: i32,}

impl Plugin for HelloPlugin {
  fn build(&self, app: &mut App) {
      app.add_systems(Update, hello_world);
  }
}

fn hello_world() {
  println!("hello world!");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,(setup, create_map))
        .add_systems(Update, pan_camera)
        .run();
}

fn setup (mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}

fn create_map(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  for q in -3..3 {
    for r in -3..3 {
      commands.spawn((
        HexTile, 
        AxialCoordinate {q: q, r: r}, 
        MaterialMesh2dBundle{
          mesh: meshes.add(shape::RegularPolygon::new(hexutils::inner_radius(), 6).into()).into(),
          material: materials.add(ColorMaterial::from(Color::GREEN)),
          transform: hexutils::hex_to_pixel(&AxialCoordinate {q: q, r: r}),
          ..default()
        }));
    }
  }
}

fn pan_camera(
  time: Res<Time>,
  mut query: Query<&mut Transform, With<Camera>>,
) {
  let mut camera_transform = query.single_mut();
  camera_transform.translation.x += 30. * time.delta_seconds();
}

mod hexutils {
  use bevy::prelude::Transform;

  use crate::AxialCoordinate;

  pub const OUTER_RADIUS: f32 = 64.0;

  pub fn inner_radius() -> f32 {
    (3.0_f32.sqrt() / 2.0) * OUTER_RADIUS
  }

  pub fn hex_to_pixel(hex: &AxialCoordinate) -> Transform {
    let x = OUTER_RADIUS * (3.0_f32.sqrt() * hex.q as f32 + (3.0_f32.sqrt() / 2.0) * hex.r as f32);
    let y = OUTER_RADIUS * (3.0 / 2.0 * hex.r as f32);
    Transform::from_xyz(x, y, 0.0)
  }
}