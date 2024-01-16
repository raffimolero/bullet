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
            (hit_enemy_with_bullet, hit_enemy)
                .chain()
                .run_if(in_state(GState::InGame)),
        );
    }
}

#[derive(Component, Clone, Copy)]
pub struct Enemy;

fn hit_enemy_with_bullet(
    mut hit: Query<
        (Entity, &Transform, &HitRadius, &mut DamageTaken),
        (With<Enemy>, Without<Ghost>),
    >,
    hitters: Query<
        (&Transform, &HitRadius, &BodyDamage),
        (With<Player>, With<Bullet>, Without<Ghost>),
    >,
) {
    for (a_id, a_tf, a_hr, a_dmg_tkn) in hit.iter() {
        for (b_tf, b_hr, b_dmg) in hitters.iter() {
            let c = a_hr.0 + b_hr.0;
            let d2 = a_tf
                .translation
                .truncate()
                .distance_squared(b_tf.translation.truncate());
            if d2 < c * c {
                a_dmg_tkn.0 += b_dmg.0;
            }
        }
    }
}
