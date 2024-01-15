use crate::game::prelude::*;

use bevy::prelude::*;

pub mod prelude {
    pub use super::Enemy;
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEnemy>()
            .add_event::<EnemyDeath>()
            .add_systems(
                Update,
                (hit_enemy_with_bullet, hit_enemy, enemy_death)
                    .chain()
                    .run_if(in_state(GState::InGame)),
            );
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnemyType {
    Dart,
}

impl From<EnemyType> for Hp {
    fn from(value: EnemyType) -> Self {
        use EnemyType::*;
        Self(match value {
            Dart => 10,
        })
    }
}

#[derive(Component)]
pub struct Enemy;

fn hit_enemy_with_bullet(
    hit: Query<(Entity, &Transform, &HitRadius), (With<Enemy>, Without<Ghost>)>,
    hitters: Query<
        (&Transform, &HitRadius, &BodyDamage),
        (With<Player>, With<Bullet>, Without<Ghost>),
    >,
    mut hit_events: EventWriter<DamageEnemy>,
) {
    for (a_id, a_tf, a_hr) in hit.iter() {
        for (b_tf, b_hr, b_dmg) in hitters.iter() {
            let c = a_hr.0 + b_hr.0;
            let d2 = a_tf
                .translation
                .truncate()
                .distance_squared(b_tf.translation.truncate());
            if d2 < c * c {
                hit_events.send(DamageEnemy(a_id, b_dmg.0));
            }
        }
    }
}

#[derive(Event, Debug, Clone, Copy)]
pub struct DamageEnemy(Entity, i32);

fn hit_enemy(
    mut commands: Commands,
    mut enemy: Query<(Entity, &mut Hp, &mut Sprite), With<Enemy>>,
    mut hit_events: EventReader<DamageEnemy>,
    mut death_events: EventWriter<EnemyDeath>,
) {
    // TODO
    /*
    let Ok((p_id, mut hp, mut sprite)) = enemy.get_single_mut() else {
        return;
    };
    let dmg = hit_events.into_iter().map(|dmg| dmg.0).max().unwrap_or(0);
    if dmg == 0 {
        return;
    }

    commands
        .entity(p_id)
        .insert((IFramePack::new(sprite.color).bundle(),));

    hp.0 -= dmg;
    if hp.0 <= 0 {
        death_events.send(EnemyDeath)
    }
    */
}

#[derive(Event)]
struct EnemyDeath(Entity);

fn enemy_death(
    mut commands: Commands,
    mut death_events: EventReader<EnemyDeath>,
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
