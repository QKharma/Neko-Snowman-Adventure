use benimator::*;
use bevy::{prelude::*, render::camera::*};
use bevy_asset_loader::AssetLoader;

extern crate nalgebra as na;
use na::Vector2;
use ncollide2d::shape::Cuboid;

mod player;
mod sprites;
mod colliders;
mod window;

use player::*;
use sprites::*;
use colliders::*;
use window::*;

fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  windows: ResMut<Windows>,
) {
  let sign_handle = asset_server.load("sign.png");
  let border_handle = asset_server.load("border.png");
  let mut camera = OrthographicCameraBundle::new_2d();
  let window = windows.get_primary().unwrap();
  camera.orthographic_projection.window_origin = WindowOrigin::Center;
  camera.orthographic_projection.scaling_mode = ScalingMode::WindowSize;
  camera.orthographic_projection.scale = (1920.0 / window.width()) * 0.25;
  println!(
    "w:{} h:{}",
    camera.orthographic_projection.scale * window.width(),
    camera.orthographic_projection.scale * window.height()
  );

  commands.spawn_bundle(camera);
  commands.spawn().insert_bundle(SpriteBundle {
    material: materials.add(border_handle.into()),
    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.5)),
    ..Default::default()
  });
  commands
    .spawn()
    .insert_bundle(SpriteBundle {
      material: materials.add(sign_handle.into()),
      transform: Transform::from_translation(Vec3::new(-30.0, 30.0, 1.0)),
      ..Default::default()
    })
    .insert(MoveBehind)
    .insert(Collider)
    .insert(Interactable)
    .insert(RectCollider(Cuboid::new(Vector2::new(2.0, 3.0))));
}

fn main() {
  let mut app = App::build();
  AssetLoader::new(GameState::AssetLoading, GameState::Next)
    .with_collection::<SpriteHandles>()
    .build(&mut app);
  app
    .add_state(GameState::AssetLoading)
    .add_plugins(DefaultPlugins)
    .add_plugin(WindowPlugin)
    .add_plugin(AnimationPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(SpritePlugin)
    .add_system_set(SystemSet::on_enter(GameState::Next).with_system(setup.system()))
    .run();
}
