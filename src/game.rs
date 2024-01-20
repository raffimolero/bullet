use crate::prelude::*;

pub mod prelude {
    pub use super::{
        blocc::prelude::*, level::prelude::*, logic::prelude::*, mob::prelude::*,
        motion::prelude::*, pack::prelude::*,
    };
}

pub mod blocc;
pub mod level;
pub mod logic;
pub mod mob;
pub mod motion;
pub mod pack;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugins((motion::Plug, logic::Plug, level::Plug, mob::Plug));
    }
}
