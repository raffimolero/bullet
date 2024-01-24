use crate::prelude::*;

pub mod prelude {
    pub use super::{Control, PlayerDeath};
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDeath>().add_systems(
            Update,
            (
                control.in_set(GameLoop::Control),
                player_death.in_set(GameLoop::Meta),
            ),
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

// FIXME: no movement.
// possible causes:
// data not added
// control too weak
// system not run
fn control(
    time: Res<Time>,
    mut player: Query<
        (
            Option<&MaxAccel>,
            Option<&mut Acceleration>,
            Option<&mut Target>,
            Option<&mut WeaponState>,
        ),
        With<Control>,
    >,
    cursor_tracker: Query<&GlobalTransform, With<CursorTracker>>,
    clicks: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
) {
    let Ok((max_accel, accel, target, wpn_st)) = player.get_single_mut() else {
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
    // TODO: when adding static object bounciness, player should be able to shake a bit.
    mov = spd * mov.normalize_or_zero() * max_accel.map_or(0.0, |max_accel| max_accel.speed);
    dbg!(max_accel);

    let spd = TAU / 2.0;
    let mut rot = 0.0;
    if keys.pressed(KeyCode::E) {
        rot += 1.0;
    }
    if keys.pressed(KeyCode::Q) {
        rot -= 1.0;
    }
    rot *= spd;

    if let Some(mut accel) = accel {
        *accel = Acceleration {
            velocity: mov,
            rotation: rot,
            // maybe some other time we'll have growth controls
            growth: 0.0,
        };
        dbg!(accel);
    }
    if let Some(mut wpn_st) = wpn_st {
        wpn_st.firing = keys.pressed(KeyCode::Space) || clicks.pressed(MouseButton::Left);
        dbg!(wpn_st);
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
