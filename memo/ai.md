use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

// GameAssets の定義（既存のコード）
#[derive(Resource, AssetCollection)]
pub struct GameAssets {
    #[asset(path = "player/plane1.png")]
    pub player_1: Handle<Image>,
    #[asset(path = "player/plane2.png")]
    pub player_2: Handle<Image>,
    #[asset(path = "player/plane_white.png")]
    pub player_white: Handle<Image>,
}

// ゲームの状態を表す列挙型
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    AssetLoading,
    Playing,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Playing)
        )
        .add_collection_to_loading_state::<_, GameAssets>(GameState::AssetLoading)
        .add_systems(
            OnEnter(GameState::Playing),
            setup_game
        )
        .add_systems(
            Update,
            (
                spawn_players,
                move_player,
                shoot_projectile,
                move_projectiles,
                destroy_projectiles,
                check_for_collisions,
            ).run_if(in_state(GameState::Playing))
        )
        .run();
}

fn setup_game(mut commands: Commands) {
    // ゲームの初期設定を行う
    // 例: カメラの設定など
    commands.spawn(Camera2dBundle::default());
}

// 他のシステム関数（spawn_players, move_player など）はここに実装