use crate::prelude::*;

pub mod prelude {
    pub use super::{
        blocc::prelude::*, level::prelude::*, logic::prelude::*, mob::prelude::*, pack::prelude::*,
    };
}

pub mod blocc;
pub mod level;
pub mod logic;
pub mod mob;
pub mod pack;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugins((logic::Plug, level::Plug, mob::Plug));
    }
}
