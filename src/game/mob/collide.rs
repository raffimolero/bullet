use crate::prelude::*;

pub mod prelude {
    pub use super::{Enemy, Neutral, Player, Team};
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (hit_enemy_with_bullet, hit_player_with_bullet).in_set(GameLoop::CollideCheck),
        );
    }
}

// TODO: Arc<()> per team
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Team {
    Player,
    Neutral,
    Enemy,
}

impl Pack for Team {
    fn attach(self, commands: &mut EntityCommands) {
        match self {
            Team::Player => commands.insert((self, Player)),
            Team::Neutral => commands.insert((self, Neutral)),
            Team::Enemy => commands.insert((self, Enemy)),
        };
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Player;
#[derive(Component, Debug, Clone, Copy)]
pub struct Neutral;
#[derive(Component, Debug, Clone, Copy)]
pub struct Enemy;

fn hit_enemy_with_bullet(
    hit: Query<(Entity, &GlobalTransform, &HitRadius), (With<Enemy>, Without<Ghost>)>,
    hitters: Query<(Entity, &GlobalTransform, &HitRadius), (With<Player>, Without<Ghost>)>,
    mut hit_events: EventWriter<MobHit>,
) {
    for (a_id, a_gtf, a_hr) in hit.iter() {
        for (b_id, b_gtf, b_hr) in hitters.iter() {
            let (a_scl, _a_rot, a_tl) = a_gtf.to_scale_rotation_translation();
            let (b_scl, _b_rot, b_tl) = b_gtf.to_scale_rotation_translation();
            let c = a_scl.x.max(a_scl.y) * a_hr.0 + b_scl.x.max(b_scl.y) * b_hr.0;
            let d2 = a_tl.truncate().distance_squared(b_tl.truncate());
            if d2 < c * c {
                hit_events.send(MobHit {
                    hit: a_id,
                    hitter: b_id,
                })
            }
        }
    }
}

fn hit_player_with_bullet(
    hit: Query<(Entity, &GlobalTransform, &HitRadius), (With<Player>, Without<Ghost>)>,
    hitters: Query<(Entity, &GlobalTransform, &HitRadius), (With<Enemy>, Without<Ghost>)>,
    mut hit_events: EventWriter<MobHit>,
) {
    for (a_id, a_gtf, a_hr) in hit.iter() {
        for (b_id, b_gtf, b_hr) in hitters.iter() {
            let (a_scl, _a_rot, a_tl) = a_gtf.to_scale_rotation_translation();
            let (b_scl, _b_rot, b_tl) = b_gtf.to_scale_rotation_translation();
            let c = a_scl.x.max(a_scl.y) * a_hr.0 + b_scl.x.max(b_scl.y) * b_hr.0;
            let d2 = a_tl.truncate().distance_squared(b_tl.truncate());
            if d2 < c * c {
                hit_events.send(MobHit {
                    hit: a_id,
                    hitter: b_id,
                })
            }
        }
    }
}
