use crate::game::prelude::*;

use bevy::prelude::*;

pub mod prelude {
    pub use super::Enemy;
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (hit_enemy_with_bullet).run_if(in_state(GState::InGame)),
        );
    }
}

#[derive(Component, Clone, Copy)]
pub struct Enemy;

fn hit_enemy_with_bullet(
    mut hit: Query<(Entity, &Transform, &HitRadius), (With<Enemy>, Without<Ghost>)>,
    hitters: Query<(Entity, &Transform, &HitRadius), (With<Player>, With<Bullet>, Without<Ghost>)>,
    mut hit_events: EventWriter<MobHit>,
) {
    for (a_id, a_tf, a_hr) in hit.iter_mut() {
        for (b_id, b_tf, b_hr) in hitters.iter() {
            let c = a_hr.0 + b_hr.0;
            let d2 = a_tf
                .translation
                .truncate()
                .distance_squared(b_tf.translation.truncate());
            if d2 < c * c {
                hit_events.send(MobHit {
                    hit: a_id,
                    hitter: b_id,
                })
            }
        }
    }
}
