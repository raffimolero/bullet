use crate::game::prelude::*;

use bevy::prelude::*;

pub mod prelude {
    pub use super::{Control, Player, PlayerBundle};
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_event::<DamagePlayer>()
            .add_event::<PlayerDeath>()
            .add_systems(
                Update,
                (
                    if hp.0 <= 0 {
                        return;
                    }

                    control.before(motion),
                    (hit_player_with_bullet, hit_player, player_death)
                        .after(motion)
                        .chain(),
                )
                    .run_if(in_state(GState::InGame)),
            );
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    team: Team,
    control: Control,
    weapon: Weapon,
    weapon_type: WeaponType,
    mob: Mob,
    hp: Hp,
    body_damage: BodyDamage,
    hit_radius: HitRadius,
    vel: Vel,
    sprite: SpriteBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            team: Team::Player,
            player: Player,
            control: Control,
            weapon: Weapon::default(),
            weapon_type: WeaponType::Basic,
            mob: Mob,
            hp: Hp(3),
            body_damage: BodyDamage(1),
            hit_radius: HitRadius(UNIT),
            vel: Vel(Vec2::default()),
            sprite: Blocc {
                x: 0.0,
                y: 0.0,
                w: UNIT * 2.0,
                h: UNIT * 2.0,
                color: Color::BLUE,
                ..default()
            }
            .bundle(),
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct Player;

#[derive(Component, Clone, Copy)]
pub struct Control;

fn control(mut player: Query<(&mut Vel, &mut Weapon), With<Control>>, keys: Res<Input<KeyCode>>) {
    let Ok((mut vel, mut wpn)) = player.get_single_mut() else {
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

    wpn.firing = keys.pressed(KeyCode::Space);

    vel.0 = mov;
}

fn hit_player_with_bullet(
    hit: Query<(Entity, &Transform, &HitRadius), (With<Player>, Without<Ghost>)>,
    hitters: Query<
        (&Transform, &HitRadius, &BodyDamage),
        (With<Enemy>, With<Bullet>, Without<Ghost>),
    >,
    mut hit_events: EventWriter<DamageMob>,
) {
    for (a_id, a_tf, a_hr) in hit.iter() {
        for (b_tf, b_hr, b_dmg) in hitters.iter() {
            let c = a_hr.0 + b_hr.0;
            let d2 = a_tf
                .translation
                .truncate()
                .distance_squared(b_tf.translation.truncate());
            if d2 < c * c {
                hit_events.send(DamagePlayer(b_dmg.0));
            }
        }
    }
}

#[derive(Event, Debug, Clone, Copy)]
pub struct DamagePlayer(i32);

fn hit_player(
    mut commands: Commands,
    mut player: Query<(Entity, &mut Hp, &mut Sprite, &MobType), With<Player>>,
    mut hit_events: EventReader<DamagePlayer>,
    mut death_events: EventWriter<MobDeath>,
    mut p_death_events: EventWriter<PlayerDeath>,
) {
    let Ok((p_id, mut hp, sprite, p_mob)) = player.get_single_mut() else {
        return;
    };
    if hp.0 <= 0 {
        return;
    }

    let dmg = hit_events.read().map(|dmg| dmg.0).max().unwrap_or(0);
    if dmg == 0 {
        return;
    }

    commands
        .entity(p_id)
        .insert(IFramePack::new(sprite.color).bundle());

    hp.0 -= dmg;
    if hp.0 <= 0 {
        death_events.send(MobDeath {
            id: p_id,
            mob: *p_mob,
        });
        p_death_events.send(PlayerDeath)
    }
}

#[derive(Event)]
struct PlayerDeath;

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
