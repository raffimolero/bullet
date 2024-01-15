use crate::game::prelude::*;

use bevy::prelude::*;

pub mod prelude {
    pub use super::{motion, BodyDamage, GState, HitRadius, Hp, Vel};
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_state::<GState>()
            .add_event::<Restart>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    motion.run_if(in_state(GState::InGame)),
                    (restart_bind, restart).chain(),
                )
                    .chain(),
            );
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GState {
    #[default]
    Dead,
    Waiting,
    InGame,
}

fn setup(mut commands: Commands, mut select: EventWriter<SelectLevel>) {
    commands.spawn(Camera2dBundle::default());
    select.send(SelectLevel(Level(0)));
}

#[derive(Event)]
struct Restart;

fn restart_bind(mut restart_events: EventWriter<Restart>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::R) {
        restart_events.send(Restart);
    }
}

fn restart(mut restart_events: EventReader<Restart>, mut next_state: ResMut<NextState<GState>>) {
    if restart_events.is_empty() {
        return;
    }
    restart_events.clear();

    next_state.set(GState::Waiting);
}

#[derive(Component, Deref, DerefMut)]
pub struct Vel(pub Vec2);

pub fn motion(time: Res<Time>, mut movers: Query<(&Vel, &mut Transform)>) {
    let delta = time.delta_seconds();
    movers.for_each_mut(|(vel, mut tf)| {
        tf.translation += vel.0.extend(0.0) * delta;
    });
}

#[derive(Component, Default)]
pub struct Hp(pub i32);

#[derive(Component)]
pub struct BodyDamage(pub i32);

#[derive(Component)]
pub struct HitRadius(pub f32);
