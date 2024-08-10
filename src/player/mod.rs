use bevy::prelude::*;

pub mod spawning;

use crate::RollbackState;

pub struct PlayerPlugin;

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