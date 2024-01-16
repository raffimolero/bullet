use crate::prelude::*;

use bevy::ecs::query::QuerySingleError;

pub mod prelude {
    pub use super::{
        data::prelude::*, effect::prelude::*, enemy::prelude::*, player::prelude::*, DamageTaken,
        HitDamage, HitRadius, Hp, MobDeath, MobHit, Team, UNIT,
    };
}

pub mod data;
pub mod effect;
pub mod enemy;
pub mod player;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_event::<MobHit>()
            .add_event::<MobDeath>()
            .add_plugins((player::Plug, enemy::Plug, effect::Plug))
            .add_systems(
                Update,
                (hit_mob, hurt_mob, mob_death, mob_death_2)
                    .chain()
                    .run_if(in_state(GState::InGame)),
            );
    }
}

pub const UNIT: f32 = 2.0;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Team {
    Player,
    Neutral,
    Enemy,
}

#[derive(Component, Clone, Copy)]
pub struct Neutral;

impl Team {
    pub fn attach(self, commands: &mut Commands, entity: Entity) {
        let mut cmd = commands.entity(entity);
        match self {
            Team::Player => cmd.insert((self, Player)),
            Team::Neutral => cmd.insert((self, Neutral)),
            Team::Enemy => cmd.insert((self, Enemy)),
        };
    }
}

#[derive(Component)]
pub struct Hp(pub i32);

impl Default for Hp {
    fn default() -> Self {
        Self(1)
    }
}

impl AddAssign for Hp {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Add for Hp {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

#[derive(Component, Default)]
pub struct DamageTaken(pub i32);

#[derive(Component)]
pub struct HitRadius(pub f32);

impl Default for HitRadius {
    fn default() -> Self {
        Self(5.0)
    }
}

#[derive(Component, Default)]
pub struct HitDamage(pub i32);

#[derive(Event)]
pub struct MobHit {
    pub hit: Entity,
    pub hitter: Entity,
}

fn hit_mob(
    mut commands: Commands,
    mut hit_mobs: Query<(&mut DamageTaken, &Sprite, Option<&PhaseShell>)>,
    hitter_mobs: Query<&HitDamage>,
    mut hit_events: EventReader<MobHit>,
) {
    for MobHit { hit, hitter } in hit_events.read() {
        let Ok(dmg) = hitter_mobs.get(*hitter) else {
            continue;
        };
        if dmg.0 == 0 {
            continue;
        }
        let Ok((mut dmg_tkn, sprite, shell)) = hit_mobs.get_mut(*hit) else {
            continue;
        };
        if let Some(_shell) = shell {
            commands
                .entity(*hit)
                .insert(IFramePack::new(sprite.color).bundle());
        }
        dmg_tkn.0 += dmg.0;
    }
}

fn hurt_mob(
    mut mobs: Query<(Entity, &mut DamageTaken, &mut Hp, &Mob, Option<&PhaseShell>)>,
    mut death_events: EventWriter<MobDeath>,
) {
    mobs.for_each_mut(|(id, mut dmg_tkn, mut hp, mob, shell)| {
        if dmg_tkn.0 == 0 {
            return;
        }
        if hp.0 <= 0 {
            return;
        }
        let dmg = if let Some(shell) = shell {
            shell.dmg
        } else {
            dmg_tkn.0
        };
        dmg_tkn.0 = 0;
        hp.0 -= dmg;
        if hp.0 <= 0 {
            death_events.send(MobDeath { id, mob: *mob });
        }
    })
}

#[derive(Event)]
pub struct MobDeath {
    pub id: Entity,
    pub mob: Mob,
}

fn mob_death(
    mut commands: Commands,
    mut death_events: EventReader<MobDeath>,
    mut p_death_events: EventWriter<PlayerDeath>,
    player: Query<Entity, With<Control>>,
) {
    let player = match player.get_single() {
        Ok(p) => Some(p),
        Err(QuerySingleError::NoEntities(_)) => None,
        Err(QuerySingleError::MultipleEntities(_)) => {
            panic!("assumption violated: must have at most one player")
        }
    };
    for death in death_events.read() {
        println!("id: {:?}", death.id);
        if Some(death.id) == player {
            p_death_events.send(PlayerDeath);
        }
        commands.entity(death.id).despawn_recursive();
    }
}

fn mob_death_2(mut death_events: EventReader<MobDeath>) {
    for death in death_events.read() {
        println!("id 2: {:?}", death.id);
    }
}
