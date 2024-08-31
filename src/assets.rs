use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(Resource, AssetCollection)]
pub struct GameAssets {
    #[asset(path = "player/plane1.png")]
    pub player_1: Handle<Image>,
    #[asset(path = "player/plane2.png")]
    pub player_2: Handle<Image>,
    #[asset(path = "player/plane_white.png")]
    pub player_white: Handle<Image>,
}