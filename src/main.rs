use bevy::prelude::*;
use turn_base_demo::GamePlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, GamePlugin));

    app.run();
}
