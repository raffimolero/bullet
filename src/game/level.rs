use crate::prelude::*;

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
    let Some(SelectLevel(selected_level)) = level_events.read().last() else {
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

    // let player = commands.spawn(()).attach(Control).attach(Mob::Dart).id();

    println!("{level:?}");
    // load level
    match level.0 {
        0 => {
            let dtf = DeltaTf {
                velocity: Vec2::new(0.0, -50.0),
                ..default()
            };
            commands
                .spawn(())
                .attach(Team::Enemy)
                .attach(Mob::Dart)
                .insert((dtf, Transform::from_xyz(0.0, 250.0, 0.0)));
            commands
                .spawn(())
                .attach(Team::Enemy)
                .attach(Mob::Dart)
                .insert((dtf, Transform::from_xyz(0.0, 500.0, 0.0)));
        }
        _ => {}
    }
}

fn end_level(mut commands: Commands, entities: Query<Entity, With<Mob>>) {
    entities.for_each(|id| {
        commands.entity(id).despawn_recursive();
    });
}
