use crate::game::prelude::*;

use bevy::prelude::*;

pub mod prelude {
    pub use super::{
        effect::prelude::*, enemy::prelude::*, player::prelude::*, BodyDamage, DamageMob,
        DamageTaken, HitRadius, Hp, Mob, MobDeath, MobType, Team, UNIT,
    };
}

pub mod effect;
pub mod enemy;
pub mod player;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_event::<MobDeath>()
            .add_plugins((player::Plug, enemy::Plug, effect::Plug))
            .add_systems(Update, mob_death.run_if(in_state(GState::InGame)));
    }
}

pub const UNIT: f32 = 10.0;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Team {
    Player,
    Neutral,
    Enemy,
}

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
pub struct Mob;

#[derive(Component)]
pub struct Hp(pub i32);

impl Default for Hp {
    fn default() -> Self {
        Self(1)
    }
}

#[derive(Component, Default)]
pub struct DamageTaken(pub i32);

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

#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum MobType {
    #[default]
    Dart,
}

impl From<MobType> for Hp {
    fn from(value: MobType) -> Self {
        use MobType::*;
        Self(match value {
            Dart => 10,
        })
    }
}

fn hurt_mob(
    mut commands: Commands,
    mut mobs: Query<(
        Entity,
        &DamageTaken,
        &mut Hp,
        &mut Sprite,
        &Team,
        Option<&mut PhaseShell>,
    )>,
    mut death_events: EventWriter<MobDeath>,
) {
    mobs.for_each_mut(|(id, dmg, mut hp, mut sprite, team, shell)| {
        if dmg == 0 {
            return;
        }
        if hp.0 <= 0 {
            return;
        }
        let dmg = if let Some(shell) = shell {
            commands
                .entity(p_id)
                .insert(IFramePack::new(sprite.color).bundle());
            shell.clamp(dmg)
        } else {
            dmg
        };
        hp.0 -= dmg;
        if hp.0 <= 0 {
            death_events.send(MobDeath);
        }
    })
}
#[derive(Event)]
pub struct MobDeath {
    pub id: Entity,
    pub mob: MobType,
}

fn mob_death(
    mut commands: Commands,
    mut death_events: EventReader<MobDeath>,
    enemies: Query<Entity, With<Enemy>>,
) {
    if death_events.is_empty() {
        return;
    }
    death_events.clear();

    enemies.for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}
