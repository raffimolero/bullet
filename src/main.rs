pub mod game;

pub mod prelude {
    pub use super::game::{
        blocc::prelude::*, level::prelude::*, logic::prelude::*, mob::prelude::*, pack::prelude::*,
        weapon::prelude::*,
    };
    pub use bevy::prelude::*;
    pub use std::{
        f32::consts::TAU,
        ops::{Add, AddAssign},
        time::{Duration, Instant},
    };
}
use prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, game::Plug))
        .add_systems(Update, bevy::window::close_on_esc)
        .run()
}
