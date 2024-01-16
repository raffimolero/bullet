use crate::prelude::*;

pub mod blocc;
pub mod level;
pub mod logic;
pub mod mob;
pub mod pack;
pub mod weapon;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugins((logic::Plug, level::Plug, mob::Plug, weapon::Plug));
    }
}
