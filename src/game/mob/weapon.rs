use crate::prelude::*;

pub mod prelude {
    pub use super::{Weapon, WeaponState};
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (fire).run_if(in_state(GState::InGame)));
    }
}

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

impl Pack for Weapon {
    fn attach(self, commands: &mut EntityCommands) {
        commands.insert((self, WeaponState::default(), Target::default()));
    }
}

impl Weapon {
    pub fn fire(self, commands: &mut Commands, transform: Transform, team: Team) {
        match self {
            Weapon::Basic => {
                let vel = transform.with_translation(Vec3::ZERO) * Vec3::new(0.0, 50.0, 0.0);
                commands.spawn(()).attach(Mob::Pellet).attach(team).insert((
                    transform,
                    Velocity {
                        velocity: vel.truncate(),
                        ..default()
                    },
                ));
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
    weapons.for_each_mut(|(tf, mut wpn_st, wpn, team)| {
        if wpn_st.firing && now > wpn_st.next_available {
            wpn.fire(&mut commands, *tf, *team);
            wpn_st.next_available = now + wpn.cooldown();
        }
    });
}