use super::prelude::*;

use bevy::prelude::*;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_event::<Hit>()
            .add_event::<Death>()
            .add_event::<Restart>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    (velocity, collide, damage, die)
                        .chain()
                        .run_if(in_state(GState::InGame)),
                    (restart_bind, restart).chain(),
                )
                    .chain(),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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

fn velocity(time: Res<Time>, mut movers: Query<(&Vel, &mut Transform)>) {
    let delta = time.delta_seconds();
    movers.for_each_mut(|(vel, mut tf)| {
        tf.translation += vel.0.extend(0.0);
    });
}

#[derive(Component, Default)]
pub struct Hp(pub i32);

#[derive(Component)]
pub struct HitRadius(pub f32);

#[derive(Component)]
pub struct BodyDamage(pub i32);

fn collide(
    mut colliders: Query<
        (Entity, &Transform, &HitRadius, &mut Hp, Option<&BodyDamage>),
        Without<Ghost>,
    >,
    mut hit_events: EventWriter<Hit>,
    mut death_events: EventWriter<Death>,
) {
    let mut pairs = colliders.iter_combinations_mut();
    // `b` hits `a`
    while let Some([(a_id, a_tf, a_hr, mut a_hp, _), (_, b_tf, b_hr, _, b_dmg)]) =
        pairs.fetch_next()
    {
        let c = a_hr.0 + b_hr.0;
        let d2 = a_tf
            .translation
            .truncate()
            .distance_squared(b_tf.translation.truncate());
        if d2 < c * c {
            hit_events.send(Hit(a_id));
            if let Some(dmg) = b_dmg {
                a_hp.0 -= dmg.0;
                if a_hp.0 <= 0 {
                    death_events.send(Death(a_id))
                }
            }
            break;
        }
    }
}

#[derive(Event, Debug)]
pub struct Hit(Entity);

fn damage(mut hit_events: EventReader<Hit>) {
    for hit in hit_events.into_iter() {
        println!("{hit:?} was hit");
    }
}

#[derive(Event)]
struct Death(Entity);

fn die(
    mut commands: Commands,
    mut death_events: EventReader<Death>,
    player: Query<Entity, With<Player>>,
) {
    if death_events.is_empty() {
        return;
    }
    death_events.clear();

    commands.entity(player.single()).despawn_recursive();
}
