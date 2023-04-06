use crate::player::comp::Player;

use super::comp::*;
use bevy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/fishing_rod.glb#Scene0"),
            ..default()
        },
        FishingRod,
    ));
}

pub fn update_rod_pos(
    mut commands: Commands,
    mut fishing_rod_query: Query<&mut Transform, (With<FishingRod>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<FishingRod>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut fishing_rod_transform) = fishing_rod_query.get_single_mut() {
            fishing_rod_transform.translation =
                player_transform.translation + player_transform.forward() * 3.;
        }
    }
}
