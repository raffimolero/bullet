use crate::prelude::*;

pub mod prelude {
    pub use super::{Control, Player, PlayerDeath};
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDeath>().add_systems(
            Update,
            (
                control.before(motion),
                (hit_player_with_bullet.before(super::hit_mob), player_death)
                    .after(motion)
                    .chain(),
            )
                .run_if(in_state(GState::InGame)),
        );
    }
}

#[derive(Component, Clone, Copy)]
pub struct Player;

#[derive(Component, Clone, Copy)]
pub struct Control;

impl Pack for Control {
    fn attach(self, commands: &mut Commands, entity: Entity) {
        commands
            .entity(entity)
            .insert((self, PhaseShell::default()));
        Team::Player.attach(commands, entity);
    }
}

fn control(
    mut player: Query<(&mut Vel, &mut WeaponState), With<Control>>,
    keys: Res<Input<KeyCode>>,
) {
    let Ok((mut vel, mut wpn)) = player.get_single_mut() else {
        return;
    };

    let spd = 200.0;
    let mut mov = Vec2::ZERO;
    if keys.pressed(KeyCode::W) {
        mov.y += 1.0;
    }
    if keys.pressed(KeyCode::S) {
        mov.y -= 1.0;
    }
    if keys.pressed(KeyCode::D) {
        mov.x += 1.0;
    }
    if keys.pressed(KeyCode::A) {
        mov.x -= 1.0;
    }
    mov = spd * mov.normalize_or_zero();

    wpn.firing = keys.pressed(KeyCode::Space);

    vel.0 = mov;
}

// TODO: GlobalTransform
fn hit_player_with_bullet(
    hit: Query<(Entity, &Transform, &HitRadius), (With<Player>, Without<Ghost>)>,
    hitters: Query<(Entity, &Transform, &HitRadius), (With<Enemy>, Without<Ghost>)>,
    mut hit_events: EventWriter<MobHit>,
) {
    for (a_id, a_tf, a_hr) in hit.iter() {
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

#[derive(Event)]
pub struct PlayerDeath;

fn player_death(
    mut commands: Commands,
    mut death_events: EventReader<PlayerDeath>,
    player: Query<Entity, With<Player>>,
) {
    if death_events.is_empty() {
        return;
    }
    death_events.clear();
}
