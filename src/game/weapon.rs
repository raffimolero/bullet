use crate::game::prelude::*;

use bevy::prelude::*;

pub mod prelude {
    pub use super::{Bullet, Weapon, WeaponState};
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (fire).run_if(in_state(GState::InGame)));
    }
}

#[derive(Component, Clone, Copy)]
pub struct Bullet;

#[derive(Component, Clone, Copy)]
pub struct WeaponState {
    pub next_available: Instant,
    pub firing: bool,
}

impl Default for WeaponState {
    fn default() -> Self {
        Self {
            next_available: Instant::now(),
            firing: false,
        }
    }
}

#[derive(Component, Clone, Copy, Default)]
pub enum Weapon {
    #[default]
    Basic,
}

impl Weapon {
    pub fn fire(self, commands: &mut Commands, transform: Transform, team: Team) {
        match self {
            Weapon::Basic => {
                let mut blocc = Blocc {
                    w: UNIT * 2.0,
                    h: UNIT * 2.0,
                    color: Color::RED,
                    ..default()
                }
                .bundle();
                blocc.transform = transform;
                let vel = transform.with_translation(Vec3::ZERO) * Vec3::new(0.0, 50.0, 0.0);
                let bullet = commands
                    .spawn((
                        Bullet,
                        HitDamage(1),
                        Mob::Pellet,
                        HitRadius(UNIT),
                        blocc,
                        Vel(vel.truncate()),
                    ))
                    .id();
                team.attach(commands, bullet);
            }
        }
    }

    pub fn cooldown(self) -> Duration {
        Duration::from_secs_f32(match self {
            Weapon::Basic => 1.0,
        })
    }
}

fn fire(
    mut commands: Commands,
    mut weapons: Query<(&Transform, &mut WeaponState, &Weapon, &Team)>,
) {
    let now = Instant::now();
    weapons.for_each_mut(|(tf, mut wpn, wpn_ty, team)| {
        if wpn.firing && now > wpn.next_available {
            wpn_ty.fire(&mut commands, *tf, *team);
            wpn.next_available = now + wpn_ty.cooldown();
        }
    });
}
