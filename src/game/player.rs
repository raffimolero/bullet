use super::prelude::*;

use bevy::prelude::*;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_event::<HitDamage>()
            .add_event::<PlayerDeath>()
            .add_systems(
                Update,
                (
                    control.before(motion),
                    ghost,
                    flash,
                    (hit_player_with_bullet, hit_player, die)
                        .after(motion)
                        .chain(),
                )
                    .run_if(in_state(GState::InGame)),
            );
    }
}

#[derive(Component)]
pub struct Player;

fn control(mut player: Query<&mut Vel, With<Player>>, keys: Res<Input<KeyCode>>) {
    let Ok(mut vel) = player.get_single_mut() else {
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

    vel.0 = mov;
}

#[derive(Component)]
pub struct Ghost {
    expiry: Instant,
}

impl Ghost {
    pub fn from_secs(secs: f32) -> Self {
        Self {
            expiry: Instant::now() + Duration::from_secs_f32(secs),
        }
    }
}

fn ghost(mut commands: Commands, time: Res<Time>, mut objects: Query<(Entity, &Ghost)>) {
    let now = Instant::now();
    objects.for_each_mut(|(entity, ghost)| {
        if now > ghost.expiry {
            commands.entity(entity).remove::<Ghost>();
        }
    });
}

#[derive(Component)]
pub struct Flashing {
    old_color: Color,
    col_a: Color,
    col_b: Color,
    time_per_flash: Duration,
    next_flash: Instant,
    expiry: Instant,
}

impl Flashing {
    fn new(
        old_color: Color,
        col_a: Color,
        col_b: Color,
        secs_per_flash: f32,
        duration_secs: f32,
    ) -> Self {
        let now = Instant::now();
        Self {
            old_color,
            col_a,
            col_b,
            time_per_flash: Duration::from_secs_f32(secs_per_flash),
            next_flash: now,
            expiry: now + Duration::from_secs_f32(duration_secs),
        }
    }
}

fn flash(mut commands: Commands, mut objects: Query<(Entity, &mut Flashing, &mut Sprite)>) {
    let now = Instant::now();
    objects.for_each_mut(|(entity, mut flash, mut sprite)| {
        if now > flash.expiry {
            sprite.color = flash.old_color;
            commands.entity(entity).remove::<Flashing>();
            return;
        }
        if now > flash.next_flash {
            let time_per_flash = flash.time_per_flash;
            flash.next_flash += time_per_flash;
            sprite.color = flash.col_a;
            flash.col_a = flash.col_b;
            flash.col_b = sprite.color;
        }
    });
}

struct IFramePack {
    duration_secs: f32,
    old_color: Color,
    col_a: Color,
    col_b: Color,
    secs_per_flash: f32,
}

impl Default for IFramePack {
    fn default() -> Self {
        Self {
            duration_secs: 1.0,
            old_color: Color::ORANGE,
            col_a: Color::WHITE,
            col_b: Color::GRAY,
            secs_per_flash: 0.1,
        }
    }
}

impl IFramePack {
    fn new(old_color: Color) -> Self {
        Self {
            old_color,
            ..default()
        }
    }

    fn bundle(self) -> impl Bundle {
        let Self {
            duration_secs,
            old_color,
            col_a,
            col_b,
            secs_per_flash,
        } = self;
        (
            Ghost::from_secs(duration_secs),
            Flashing::new(old_color, col_a, col_b, secs_per_flash, duration_secs),
        )
    }
}

fn hit_player_with_bullet(
    hit: Query<(Entity, &Transform, &HitRadius), (With<Player>, Without<Ghost>)>,
    hitters: Query<
        (&Transform, &HitRadius, &BodyDamage),
        (With<Enemy>, With<Bullet>, Without<Ghost>),
    >,
    mut hit_events: EventWriter<HitDamage>,
) {
    for (a_id, a_tf, a_hr) in hit.iter() {
        for (b_tf, b_hr, b_dmg) in hitters.iter() {
            let c = a_hr.0 + b_hr.0;
            let d2 = a_tf
                .translation
                .truncate()
                .distance_squared(b_tf.translation.truncate());
            if d2 < c * c {
                hit_events.send(HitDamage(b_dmg.0));
            }
        }
    }
}

#[derive(Event, Debug, Clone, Copy)]
pub struct HitDamage(i32);

fn hit_player(
    mut commands: Commands,
    mut player: Query<(Entity, &mut Hp, &mut Sprite), With<Player>>,
    mut hit_events: EventReader<HitDamage>,
    mut death_events: EventWriter<PlayerDeath>,
) {
    let Ok((p_id, mut hp, mut sprite)) = player.get_single_mut() else {
        return;
    };
    let dmg = hit_events.into_iter().map(|dmg| dmg.0).max().unwrap_or(0);
    if dmg == 0 {
        return;
    }

    commands
        .entity(p_id)
        .insert((IFramePack::new(sprite.color).bundle(),));

    hp.0 -= dmg;
    if hp.0 <= 0 {
        death_events.send(PlayerDeath)
    }
}

#[derive(Event)]
struct PlayerDeath;

fn die(
    mut commands: Commands,
    mut death_events: EventReader<PlayerDeath>,
    player: Query<Entity, With<Player>>,
) {
    if death_events.is_empty() {
        return;
    }
    death_events.clear();

    commands.entity(player.single()).despawn_recursive();
}
