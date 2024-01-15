use crate::game::prelude::*;

use bevy::prelude::*;

pub mod prelude {
    pub use super::{Level, SelectLevel};
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectLevel>()
            .insert_resource(Level(0))
            .add_systems(Update, select_level)
            .add_systems(OnEnter(GState::Waiting), start_level)
            .add_systems(OnExit(GState::InGame), end_level);
    }
}

#[derive(Event, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SelectLevel(pub Level);

fn select_level(
    mut level_events: EventReader<SelectLevel>,
    mut level: ResMut<Level>,
    mut next_state: ResMut<NextState<GState>>,
) {
    let Some(SelectLevel(selected_level)) = level_events.into_iter().last() else {
        return;
    };
    *level = *selected_level;
    next_state.set(GState::Waiting);
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Level(pub u16);

fn start_level(
    mut commands: Commands,
    level: Res<Level>,
    mut next_state: ResMut<NextState<GState>>,
) {
    next_state.set(GState::InGame);

    // spawn player
    commands.spawn(PlayerBundle::default());

    println!("{level:?}");
    // load level
    match level.0 {
        0 => {
            commands.spawn((
                Enemy,
                Bullet,
                BodyDamage(1),
                Mob,
                HitRadius(UNIT),
                Blocc {
                    x: 0.0,
                    y: 250.0,
                    w: UNIT * 2.0,
                    h: UNIT * 2.0,
                    color: Color::RED,
                    ..default()
                }
                .bundle(),
                Vel(Vec2::new(0.0, -50.0)),
            ));
            commands.spawn((
                Enemy,
                Bullet,
                BodyDamage(1),
                Mob,
                HitRadius(UNIT),
                Blocc {
                    x: 0.0,
                    y: 500.0,
                    w: UNIT * 2.0,
                    h: UNIT * 2.0,
                    color: Color::RED,
                    ..default()
                }
                .bundle(),
                Vel(Vec2::new(0.0, -50.0)),
            ));
        }
        _ => {}
    }
}

fn end_level(mut commands: Commands, entities: Query<Entity, With<Mob>>) {
    entities.for_each(|id| {
        commands.entity(id).despawn_recursive();
    });
}
