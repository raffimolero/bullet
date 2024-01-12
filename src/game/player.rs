use super::prelude::*;

use bevy::prelude::*;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (control, expire_ghost).run_if(in_state(GState::InGame)),
        );
    }
}

#[derive(Component)]
pub struct Player;

fn control(time: Res<Time>, mut player: Query<&mut Vel, With<Player>>, keys: Res<Input<KeyCode>>) {
    let Ok(mut vel) = player.get_single_mut() else {
        return;
    };

    let delta = time.delta_seconds();
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
    mov = spd * delta * mov.normalize_or_zero();

    vel.0 = mov;
}

#[derive(Component)]
pub struct Ghost {
    expiry: Instant,
}

fn expire_ghost(mut commands: Commands, time: Res<Time>, objects: Query<(Entity, &Ghost)>) {
    let now = Instant::now();
    objects.for_each(|(entity, invincibility)| {
        if now > invincibility.expiry {
            commands.entity(entity).remove::<Ghost>();
        }
    });
}
