use core::str;

use bevy::prelude::*;

pub mod spawning;

use crate::RollbackState;

#[derive(Reflect, Clone, Default)]
pub struct PlayerStats {}

#[derive(Component,Reflect,Default)]

pub struct Player{
    pub handle: usize,

    pub stats:PlayerStats,
}

impl Player {
    pub fn new(handle: usize, stats: PlayerStats) -> Self {
        Self {
            handle,
            stats,
        }
    }
}

pub struct PlayerPlugin;

#[derive(Resource, Default)]
pub struct  PersistentPlayerStats {
    pub stats: [PlayerStats; 2],
}

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(RollbackState::RoundStart),
            (
                spawning::spawn_players,
            ),
        );
    }
}