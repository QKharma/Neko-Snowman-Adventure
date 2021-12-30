use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum ExecLabels {
  Movement,
  Collision,
}

const SPEED: f32 = 70.0;

pub struct Player;
pub struct Velocity(pub Vec2);

pub struct Collider;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_system(player_input.system().label(ExecLabels::Movement))
      .add_system(check_collision.system().label(ExecLabels::Collision).after(ExecLabels::Movement))
      .add_system(move_player.system().after(ExecLabels::Movement).after(ExecLabels::Collision));
  }
}

fn player_input(
  keyboard_input: Res<Input<KeyCode>>,
  mut query: Query<&mut Velocity, With<Player>>,
) {
  if let Ok(mut velocity) = query.single_mut() {
    velocity.0 = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::W) {
      velocity.0.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
      velocity.0.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
      velocity.0.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
      velocity.0.x += 1.0;
    }
    if velocity.0.length() > 1.0 {
      velocity.0.x /= velocity.0.length();
      velocity.0.y /= velocity.0.length();
    }
  }
}

fn check_collision(
  mut player_query: Query<(&Transform, &Sprite, &mut Velocity), With<Player>>,
  collider_query: Query<(&Transform, &Sprite), With<Collider>>,
) {
  if let Ok((transform, sprite, mut velocity)) = player_query.single_mut() {
    for (c_transform, c_sprite) in collider_query.iter() {
      let collision = collide(
        transform.translation,
        Vec2::new(sprite.size.x*0.9,sprite.size.y*0.5),
        c_transform.translation,
        Vec2::new(c_sprite.size.x*0.9,c_sprite.size.y*0.5),
      );
      if let Some(collision) = collision {
        match collision {
          Collision::Left => if velocity.0.x > 0.0 {velocity.0.x = 0.0},
          Collision::Right => if velocity.0.x < 0.0 {velocity.0.x = 0.0},
          Collision::Top => if velocity.0.y < 0.0 {velocity.0.y = 0.0},
          Collision::Bottom => if velocity.0.y > 0.0 {velocity.0.y = 0.0},
        }
      }
    }
  }
}

fn move_player(
  time: Res<Time>,
  mut query: Query<(&mut Transform, &Velocity, &mut Sprite), With<Player>>,
) {
  if let Ok((mut transform, velocity, mut sprite)) = query.single_mut() {

    let translation = &mut transform.translation;
    translation.x += SPEED * velocity.0.x * time.delta_seconds();
    translation.y += SPEED * velocity.0.y * time.delta_seconds();

    if velocity.0.x < 0.0 {
      sprite.flip_x = true
    }
    if velocity.0.x > 0.0 {
      sprite.flip_x = false
    }
  }
}
