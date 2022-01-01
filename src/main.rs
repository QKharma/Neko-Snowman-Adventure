use benimator::*;
use bevy::{prelude::*, render::camera::*, window::WindowMode};
use bevy_asset_loader::AssetLoader;

extern crate nalgebra as na;
use na::Vector2;
use ncollide2d::shape::Cuboid;

mod player;
mod sprites;

use player::*;
use sprites::*;

fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  let sign_handle = asset_server.load("sign.png");
  let mut camera = OrthographicCameraBundle::new_2d();
  camera.orthographic_projection.window_origin = WindowOrigin::Center;
  camera.orthographic_projection.scaling_mode = ScalingMode::WindowSize;
  camera.orthographic_projection.scale = 0.25;

  commands.spawn_bundle(camera);
  commands
    .spawn()
    .insert_bundle(SpriteBundle {
      material: materials.add(sign_handle.into()),
      transform: Transform::from_translation(Vec3::new(-30.0, 30.0, 1.0)),
      ..Default::default()
    })
    .insert(Collider)
    .insert(Interactable)
    .insert(RectCollider(Cuboid::new(Vector2::new(2.0, 3.0))));
}

fn move_infront(
  player_query: Query<&Transform, With<Player>>,
  mut sprite_query: Query<&mut Transform, (Without<Player>, With<Sprite>)>,
) {
  if let Ok(transform) = player_query.single() {
    for mut c_transform in sprite_query.iter_mut() {
      if transform.translation.y > c_transform.translation.y {
        c_transform.translation.z = 3.0;
      } else {
        c_transform.translation.z = 1.0;
      }
    }
  }
}

fn window_resize_system(mut windows: ResMut<Windows>) {
  let window = windows.get_primary_mut().unwrap();
  println!("Window size was: {},{}", window.width(), window.height());
  window.set_mode(WindowMode::Windowed);
}

fn main() {
  let mut app = App::build();
  AssetLoader::new(GameState::AssetLoading, GameState::Next)
    .with_collection::<SpriteHandles>()
    .build(&mut app);
  app
    .add_state(GameState::AssetLoading)
    .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
    .add_plugins(DefaultPlugins)
    .add_plugin(AnimationPlugin)
    //.add_plugin(SpritePlugin)
    .add_plugin(PlayerPlugin)
    .add_system_set(SystemSet::on_enter(GameState::Next).with_system(setup.system()))
    .add_startup_system(window_resize_system.system())
    .add_system(move_infront.system())
    .run();
}
