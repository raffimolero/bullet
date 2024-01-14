use bevy::prelude::*;

pub mod prelude {
    pub use super::{
        blocc::Blocc,
        bullet::{Bullet, Lifespan},
        enemy::Enemy,
        level::{Level, SelectLevel},
        logic::{motion, BodyDamage, HitRadius, Hp, Vel},
        player::{Ghost, Player},
        GState,
    };
    pub use std::time::{Duration, Instant};
}
use prelude::*;

pub mod blocc;
pub mod bullet;
pub mod enemy;
pub mod level;
pub mod logic;
pub mod player;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_state::<GState>()
            .add_plugins((logic::Plug, level::Plug, player::Plug, bullet::Plug))
            .add_systems(Startup, setup);
    }
}

fn setup(mut select: EventWriter<SelectLevel>) {
    select.send(SelectLevel(Level(0)))
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GState {
    #[default]
    Dead,
    Waiting,
    InGame,
}
