use crate::game::prelude::*;

use bevy::prelude::*;

pub mod prelude {
    pub use super::{Bullet, Weapon, WeaponType};
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (fire));
    }
}

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Weapon {
    pub cooldown: Duration,
    pub next_available: Instant,
    pub firing: bool,
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            cooldown: Duration::from_secs(1),
            next_available: Instant::now(),
            firing: false,
        }
    }
}

#[derive(Component, Clone, Copy, Default)]
pub enum WeaponType {
    #[default]
    Basic,
}

fn fire() {
    // TODO
}
