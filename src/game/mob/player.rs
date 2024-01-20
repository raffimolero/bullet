use crate::prelude::*;

pub mod prelude {
    pub use super::{Control, PlayerDeath};
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDeath>().add_systems(
            Update,
            (control.in_set(Think), player_death).run_if(in_state(GState::InGame)),
        );
    }
}

// TODO: TakeOver event
#[derive(Component, Clone, Copy)]
pub struct Control;

impl Pack for Control {
    fn attach(self, commands: &mut EntityCommands) {
        commands
            .attach(Team::Player)
            .insert((self, PhaseShell::default()));
    }
}

fn control(
    time: Res<Time>,
    mut player: Query<
        (
            &mut Transform,
            Option<&mut Target>,
            Option<&mut Acceleration>,
        ),
        With<Control>,
    >,
    keys: Res<Input<KeyCode>>,
) {
    let Ok((mut tf, mut brain)) = player.get_single_mut() else {
        return;
    };

    // keyboard input
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
    mov = (tf.with_translation(Vec3::ZERO) * mov.extend(1.0)).truncate();

    let spd = TAU / 2.0;
    let mut rot = 0.0;
    if keys.pressed(KeyCode::E) {
        rot += 1.0;
    }
    if keys.pressed(KeyCode::Q) {
        rot -= 1.0;
    }
    rot *= spd;

    brain.desired_vel = mov;
    brain.desired_rot_spd = rot;
    brain.firing = keys.pressed(KeyCode::Space);
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
