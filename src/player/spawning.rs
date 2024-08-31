use bevy::prelude::*;
use crate::assets::GameAssets;
use super::{PersistentPlayerStats, PlayerStats, Player};

pub const P1_TRANSFORM: Transform = Transform {
    scale: Vec3::ONE,
    rotation: Quat::IDENTITY,
    translation: Vec3::new( -800.0, 0.0, 0.0),
};
pub const P2_TRANSFORM: Transform = Transform {
    scale: Vec3::ONE,
    rotation: Quat::from_xyzw(0.0, 0.0, 0.0, 0.0),
    translation: Vec3::new(800.0, 0.0, 0.0),
};
pub fn player_spawn_transform(handle: usize) -> Transform{
    if handle == 0{
        P1_TRANSFORM
    }else {
        P2_TRANSFORM
    }
}

fn spawn_player(
    commands: &mut Commands,
    texture: Handle<Image>,
    handle: usize,
    stats: PlayerStats,
) -> Entity {
   let transform = player_spawn_transform(handle);
   commands 
        .spawn((
            Player::new(handle, stats.clone()),
        ))
        .id()
}

pub fn spawn_players(
    mut commands :Commands,
    assets: Res<GameAssets>,
    stats: Res<PersistentPlayerStats>
) {
    let textures = [assets.player_1.clone(), assets.player_2.clone()];
    
    for (handle, texture) in textures.into_iter().enumerate(){
        let player: Entity = spawn_player(&mut commands, texture, handle, stats.stats[handle].clone());
    }
}