use crate::game::prelude::*;

use bevy::prelude::*;

pub mod prelude {
    pub use super::{effect::prelude::*, enemy::prelude::*, player::prelude::*, Mob};
}

pub mod effect;
pub mod enemy;
pub mod player;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugins((player::Plug, enemy::Plug, effect::Plug));
    }
}

#[derive(Component)]
pub struct Mob;
