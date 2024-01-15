use crate::game::prelude::*;

use bevy::prelude::*;

pub mod prelude {
    pub use super::{Bullet, Weapon, WeaponType};
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (fire).run_if(in_state(GState::InGame)));
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

impl WeaponType {
    pub fn fire(self, commands: &mut Commands, transform: Transform) {
        match self {
            WeaponType::Basic => {
                let mut blocc = Blocc {
                    w: UNIT * 2.0,
                    h: UNIT * 2.0,
                    color: Color::RED,
                    ..default()
                }
                .bundle();
                blocc.transform = transform;
                let vel = transform.with_translation(Vec3::ZERO) * Vec3::new(0.0, 50.0, 0.0);
                commands.spawn((
                    Enemy,
                    Bullet,
                    BodyDamage(1),
                    Mob,
                    HitRadius(UNIT),
                    blocc,
                    Vel(vel.truncate()),
                ));
            }
        }
    }

    pub fn cooldown(self) -> Duration {
        Duration::from_secs_f32(match self {
            WeaponType::Basic => 1.0,
        })
    }
}

fn fire(mut commands: Commands, mut weapons: Query<(&Transform, &mut Weapon, &WeaponType)>) {
    let now = Instant::now();
    weapons.for_each_mut(|(tf, mut wpn, wpn_ty)| {
        println!("test");
        if wpn.firing && now > wpn.next_available {
            wpn_ty.fire(&mut commands, *tf);
            wpn.next_available += wpn_ty.cooldown();
        }
    });
}
