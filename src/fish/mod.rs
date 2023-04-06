use self::system::*;
use bevy::prelude::*;

pub mod comp;
mod system;

pub struct FishingPlugin;

impl Plugin for FishingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(update_rod_pos);
    }
}
