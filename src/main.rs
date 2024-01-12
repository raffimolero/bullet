mod game;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, game::Plug))
        .add_systems(Update, bevy::window::close_on_esc)
        .run()
}
