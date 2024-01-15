use bevy::prelude::*;

pub mod prelude {
    pub use super::{
        blocc::prelude::*, level::prelude::*, logic::prelude::*, mob::prelude::*,
        weapon::prelude::*,
    };
    pub use std::time::{Duration, Instant};
}
use prelude::*;

pub mod blocc;
pub mod level;
pub mod logic;
pub mod mob;
pub mod weapon;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugins((logic::Plug, level::Plug, mob::Plug));
    }
}
