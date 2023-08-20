use std::{f32::consts::TAU, time::Instant};

use bevy::prelude::*;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                velocity,
                player_to_bullet_collision,
                (check_health, check_lifespan),
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    println!("Hello, World!");

    commands.spawn(Camera2dBundle::default());
    commands.spawn(PlayerBundle {
        hitbox: PlayerHitbox,
        sprite_bundle: sprite_bundle(Vec2::ZERO, Vec2::new(10.0, 10.0), Color::GREEN),
    });
    commands.spawn(BulletBundle::new(
        Vec2 { x: 0.0, y: 20.0 },
        5.0,
        5.0,
        TAU / 4.0,
    ));
}

#[derive(Bundle)]
struct PlayerBundle {
    hitbox: PlayerHitbox,
    sprite_bundle: SpriteBundle,
}

#[derive(Bundle)]
struct BulletBundle {
    bullet: Bullet,
    velocity: Velocity,
    sprite_bundle: SpriteBundle,
}

impl BulletBundle {
    fn new(pos: Vec2, radius: f32, speed: f32, angle: f32) -> Self {
        Self {
            bullet: Bullet {
                radius_sq: radius * radius,
            },
            velocity: Velocity::new_vector(speed, angle),
            sprite_bundle: sprite_bundle(pos, Vec2::splat(radius * 2.0), Color::RED),
        }
    }
}

#[derive(Component)]
struct Lifespan {
    despawn_at: Instant,
}

#[derive(Component)]
struct Health(i8);

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

impl Velocity {
    fn new_vector(magnitude: f32, radians: f32) -> Self {
        Self(Vec2::from_angle(radians) * magnitude)
    }
    fn new_coords(x: f32, y: f32) -> Self {
        Self(Vec2 { x, y })
    }
}

enum BulletPattern {
    Octagon,
}

#[derive(Component)]
struct Attack {
    // pattern:
}

impl BulletPattern {
    // fn
}

#[derive(Component)]
/// a (currently invisible) one-pixel hitbox
struct PlayerHitbox;

#[derive(Component)]
struct Bullet {
    radius_sq: f32,
}

fn velocity(mut objects: Query<(&mut Transform, &Velocity)>) {
    objects.for_each_mut(|(mut transform, velocity)| {
        transform.translation += velocity.0.extend(0.0);
    });
}

fn player_to_bullet_collision(
    mut commands: Commands,
    mut player_hitboxes: Query<(&Transform, &mut Health), With<PlayerHitbox>>,
    bullet_hitboxes: Query<(Entity, &Transform, &Bullet)>,
) {
    player_hitboxes.for_each_mut(|(player_transform, mut health)| {
        bullet_hitboxes.for_each(|(bullet_entity, bullet_transform, bullet)| {
            let Vec3 { x, y, z: _ } = player_transform.translation;
            let player_pos = Vec2::new(x, y);
            let Vec3 { x, y, z: _ } = bullet_transform.translation;
            let bullet_pos = Vec2::new(x, y);
            if player_pos.distance_squared(bullet_pos) < bullet.radius_sq {
                health.0 -= 1; // maybe should send a Damage deferred command
                commands.entity(bullet_entity).despawn();
                println!("HIT! Player health: {}hp", health.0);
            }
        });
    });
}

// TODO: spawn particles on player death, without a double free

fn check_health(mut commands: Commands, objects: Query<(Entity, &Health, &Name)>) {
    objects.for_each(|(entity, health, name)| {
        if health.0 <= 0 {
            println!("{name} has died!");
            commands.entity(entity).despawn();
        }
    })
}

fn check_lifespan(mut commands: Commands, objects: Query<(Entity, &Lifespan)>) {
    let now = Instant::now();
    objects.for_each(|(entity, lifespan)| {
        if now > lifespan.despawn_at {
            println!("despawned entity.");
            commands.entity(entity).despawn();
        }
    })
}

fn sprite_bundle(pos: Vec2, size: Vec2, color: Color) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(size),
            ..default()
        },
        transform: Transform::from_translation(pos.extend(0.0)),
        ..default()
    }
}
