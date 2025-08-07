mod attribute_system;
mod skill_system;

pub use attribute_system::*;
pub use skill_system::*;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SkillPlugin);
    }
}

mod tests {

    #[test]
    fn test_game_plugin() {
        let a = 1 + 2;
        assert_eq!(a, 3)
    }
}
