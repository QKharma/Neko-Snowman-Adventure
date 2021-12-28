use bevy::prelude::*;
use bevy::render::camera::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum ExecLabels {
    Movement,
}

const SPEED: f32 = 60.0;

struct Player;
struct Velocity(Vec2);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials:ResMut<Assets<ColorMaterial>>,
) {
    let player_handle = asset_server.load("nekoSnowFemboy.png");
    let mut camera =  OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.window_origin = WindowOrigin::Center;
    camera.orthographic_projection.scaling_mode = ScalingMode::WindowSize;
    camera.orthographic_projection.scale = 0.25;
    
    commands.spawn_bundle(camera);
    commands
        .spawn_bundle(SpriteBundle{
            material: materials.add(player_handle.into()),
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity(Vec2::ZERO));
}

fn get_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>
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
    }
}

fn move_player(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Player>>
) {
    if let Ok((mut transform, velocity)) = query.single_mut() {
        let translation = &mut transform.translation;
        translation.x += SPEED * velocity.0.x * time.delta_seconds();
        translation.y += SPEED * velocity.0.y * time.delta_seconds();
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(get_input.system().label(ExecLabels::Movement))
        .add_system(move_player.system().after(ExecLabels::Movement))
        .run();
}
