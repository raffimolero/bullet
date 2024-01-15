use crate::game::prelude::*;

use bevy::prelude::*;

pub mod prelude {
    pub use super::{
        effect::prelude::*, enemy::prelude::*, player::prelude::*, BodyDamage, HitRadius, Hp, Mob,
    };
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

#[derive(Component)]
pub struct Hp(pub i32);

impl Default for Hp {
    fn default() -> Self {
        Self(1)
    }
}

#[derive(Component)]
pub struct BodyDamage(pub i32);

impl Default for BodyDamage {
    fn default() -> Self {
        Self(1)
    }
}

#[derive(Component)]
pub struct HitRadius(pub f32);

impl Default for HitRadius {
    fn default() -> Self {
        Self(5.0)
    }
}
