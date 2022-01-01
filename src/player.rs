use bevy::prelude::*;

extern crate nalgebra as na;
use na::{Isometry2, Vector2};
use ncollide2d::query;
use ncollide2d::shape::{Ball, Cuboid};

use crate::sprites::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum ExecLabels {
  Movement,
  Collision,
}

const SPEED: f32 = 70.0;

pub struct Player;
pub struct Velocity(pub Vec2);

pub struct Collider;

pub struct BallCollider(pub Ball<f32>);
pub struct RectCollider(pub Cuboid<f32>);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_startup_system(spawn_player.system())
      .add_system(player_input.system().label(ExecLabels::Movement))
      .add_system(
        check_collision
          .system()
          .label(ExecLabels::Collision)
          .after(ExecLabels::Movement),
      )
      .add_system(
        move_player
          .system()
          .after(ExecLabels::Movement)
          .after(ExecLabels::Collision),
      );
  }
}

fn spawn_player(
  mut commands: Commands,
  sprites: Res<SpriteHandles>,
) {
  commands
    .spawn_bundle(SpriteBundle {
      material: sprites.player_idle.clone(),
      transform: Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
      ..Default::default()
    })
    .insert(Player)
    .insert(Velocity(Vec2::ZERO))
    .insert(BallCollider(Ball::new(2.0)));
}

fn player_input(
  sprites: Res<SpriteHandles>,
  keyboard_input: Res<Input<KeyCode>>,
  mut query: Query<(&mut Velocity, &mut Sprite, &mut Handle<ColorMaterial>), With<Player>>,
) {
  if let Ok((mut velocity, mut sprite, mut handle)) = query.single_mut() {
    velocity.0 = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::W) {
      velocity.0.y += 1.0;
      *handle = sprites.player_up.clone();
    }
    if keyboard_input.pressed(KeyCode::S) {
      velocity.0.y -= 1.0;
      *handle = sprites.player_down.clone();
    }
    if keyboard_input.pressed(KeyCode::A) {
      velocity.0.x -= 1.0;
      *handle = sprites.player_idle.clone();
      sprite.flip_x = true;
    }
    if keyboard_input.pressed(KeyCode::D) {
      velocity.0.x += 1.0;
      *handle = sprites.player_idle.clone();
      sprite.flip_x = false;
    }
    if velocity.0.length() > 1.0 {
      velocity.0.x /= velocity.0.length();
      velocity.0.y /= velocity.0.length();
    }
  }
}

fn check_collision(
  mut player_query: Query<(&Transform, &BallCollider, &mut Velocity), With<Player>>,
  collider_query: Query<(&Transform, &RectCollider), With<Collider>>,
) {
  if let Ok((transform, collider, mut velocity)) = player_query.single_mut() {
    for (c_transform, c_collider) in collider_query.iter() {
      let player_pos = Isometry2::new(
        Vector2::new(transform.translation.x, transform.translation.y)
          + Vector2::new(velocity.0.x, velocity.0.y),
        na::zero(),
      );
      let c_pos = Isometry2::new(
        Vector2::new(c_transform.translation.x, c_transform.translation.y),
        na::zero(),
      );
      let prediction = 1.0;

      let penetrating = query::contact(&player_pos, &collider.0, &c_pos, &c_collider.0, prediction);

      match penetrating {
        Some(_) => {
          velocity.0 = Vec2::new(
            velocity.0.x - penetrating.unwrap().normal.x,
            velocity.0.y - penetrating.unwrap().normal.y,
          );
        }
        None => {}
      }
    }
  }
}

fn move_player(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity), With<Player>>) {
  if let Ok((mut transform, velocity)) = query.single_mut() {
    let translation = &mut transform.translation;
    translation.x += SPEED * velocity.0.x * time.delta_seconds();
    translation.y += SPEED * velocity.0.y * time.delta_seconds();
  }
}
