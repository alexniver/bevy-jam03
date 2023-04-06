use bevy::prelude::*;

mod system;

use self::system::*;

pub struct LakePlugin;

impl Plugin for LakePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}
