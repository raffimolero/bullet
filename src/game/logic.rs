use crate::prelude::*;

pub mod prelude {
    pub use super::{CursorPos, CursorTracker, MainCam};
    pub use bevy::window::PrimaryWindow;
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_state::<GState>()
            .add_event::<Restart>()
            .init_resource::<CursorPos>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    (update_cursor, track_cursor).chain(),
                    (restart_bind, restart).chain(),
                )
                    .in_set(GameLoop::Meta),
            );
    }
}

#[derive(Component, Debug)]
pub struct MainCam;

fn setup(mut commands: Commands, mut select: EventWriter<SelectLevel>) {
    commands.spawn((MainCam, Camera2dBundle::default()));
    commands.spawn((
        CursorTracker,
        Blocc {
            w: UNIT,
            h: UNIT,
            color: Color::YELLOW,
            ..default()
        }
        .bundle(),
    ));
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

#[derive(Resource, Clone, Copy, Default)]
pub struct CursorPos {
    prev: Vec2,
    pos: Vec2,
}

fn update_cursor(window: Query<&Window, With<PrimaryWindow>>, mut mouse: ResMut<CursorPos>) {
    let window = window.single();
    let Some(mut cursor) = window.cursor_position() else {
        return;
    };
    let resolution = &window.resolution;
    let size = Vec2::new(resolution.width(), resolution.height());
    cursor -= size / 2.0;
    cursor.y *= -1.0;
    mouse.prev = mouse.pos;
    mouse.pos = cursor;
}

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct CursorTracker;

fn track_cursor(
    mut trackers: Query<&mut Transform, With<CursorTracker>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&GlobalTransform, &Camera), With<MainCam>>,
) {
    let window = window.single();
    let Some(pos) = window.cursor_position() else {
        return;
    };
    let Ok((cam_gtf, cam)) = camera.get_single() else {
        return;
    };
    let Some(world_cursor_pos) = cam.viewport_to_world_2d(cam_gtf, pos) else {
        return;
    };
    trackers.for_each_mut(|mut tf| {
        tf.translation.x = world_cursor_pos.x;
        tf.translation.y = world_cursor_pos.y;
    });
}
