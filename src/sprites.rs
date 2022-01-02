use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

use crate::player::{Player};

pub struct MoveBehind;

#[derive(Default, AssetCollection)]
pub struct SpriteHandles {
  #[asset(color_material)]
  #[asset(path = "nekoSnowFemboy.png")]
  pub player_idle: Handle<ColorMaterial>,

  #[asset(color_material)]
  #[asset(path = "nekoSnowFemboy_front.png")]
  pub player_down: Handle<ColorMaterial>,

  #[asset(color_material)]
  #[asset(path = "nekoSnowFemboy_back.png")]
  pub player_up: Handle<ColorMaterial>,
}

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.add_system(move_infront.system());
  }
}

fn move_infront(
  player_query: Query<&Transform, With<Player>>,
  mut sprite_query: Query<&mut Transform, (Without<Player>, With<MoveBehind>)>,
) {
  if let Ok(transform) = player_query.single() {
    for mut sprite_transform in sprite_query.iter_mut() {
      if transform.translation.y > sprite_transform.translation.y {
        sprite_transform.translation.z = 3.0;
      } else {
        sprite_transform.translation.z = 1.0;
      }
    }
  }
}

//save for runtime level resource loading
/*
pub struct SpritePlugin;

impl Plugin for SpritePlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      //TODO: don't init resource
      .init_resource::<SpriteHandles>()
      .add_startup_system(load_starting_assets.system());
  }
}

pub fn load_starting_assets(
  asset_server: Res<AssetServer>,
  mut sprites: ResMut<SpriteHandles>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  sprites.player_idle = materials.add(asset_server.load("nekoSnowFemboy.png").into());
  sprites.player_down = materials.add(asset_server.load("nekoSnowFemboy_front.png").into());
  sprites.player_up = materials.add(asset_server.load("nekoSnowFemboy_back.png").into());
}
*/
