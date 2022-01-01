use bevy::prelude::*;

#[derive(Default)]
pub struct SpriteHandles{
  pub player_idle: Handle<ColorMaterial>,
  pub player_down: Handle<ColorMaterial>,
  pub player_up: Handle<ColorMaterial>
}

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<SpriteHandles>()
      .add_startup_system(assign_handles.system());
  }
}

fn assign_handles(
  asset_server: Res<AssetServer>,
  mut sprites: ResMut<SpriteHandles>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  sprites.player_idle = materials.add(asset_server.load("nekoSnowFemboy.png").into());
  sprites.player_down = materials.add(asset_server.load("nekoSnowFemboy_front.png").into());
  sprites.player_up = materials.add(asset_server.load("nekoSnowFemboy_back.png").into());
}