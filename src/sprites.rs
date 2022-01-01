use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

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
