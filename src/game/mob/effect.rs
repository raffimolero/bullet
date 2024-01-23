use crate::prelude::*;

pub mod prelude {
    pub use super::{Dying, Flashing, Ghost, IFramePack, Lifespan, PhaseShell};
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (ghost, flash, lifespan).in_set(GameLoop::Effect));
    }
}

#[derive(Component)]
pub struct Dying;

#[derive(Component)]
pub struct Lifespan(pub Instant);

fn lifespan(
    mut commands: Commands,
    objects: Query<(Entity, &Lifespan, Option<&Mob>)>,
    mut death_events: EventWriter<MobDeath>,
) {
    let now = Instant::now();
    objects.for_each(|(id, lifespan, mob)| {
        if now <= lifespan.0 {
            return;
        }
        match mob {
            Some(mob) => death_events.send(MobDeath { id, mob: *mob }),
            None => commands.entity(dbg!(id)).despawn_recursive(),
        }
    });
}

/// A status effect that gives IFrames when hit.
#[derive(Component, Clone, Copy)]
pub struct PhaseShell {
    pub dmg: i32,
    pub duration: Duration,
}

impl Default for PhaseShell {
    fn default() -> Self {
        Self {
            dmg: 1,
            duration: Duration::from_secs(1),
        }
    }
}

#[derive(Component)]
pub struct Ghost {
    pub expiry: Instant,
}

impl Ghost {
    pub fn from_secs(secs: f32) -> Self {
        Self {
            expiry: Instant::now() + Duration::from_secs_f32(secs),
        }
    }
}

fn ghost(mut commands: Commands, mut objects: Query<(Entity, &Ghost)>) {
    let now = Instant::now();
    objects.for_each_mut(|(entity, ghost)| {
        if now > ghost.expiry {
            commands.entity(entity).remove::<Ghost>();
        }
    });
}

#[derive(Component)]
pub struct Flashing {
    pub old_color: Color,
    pub col_a: Color,
    pub col_b: Color,
    pub time_per_flash: Duration,
    pub next_flash: Instant,
    pub expiry: Instant,
}

impl Flashing {
    pub fn new(
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

pub struct IFramePack {
    pub duration_secs: f32,
    pub old_color: Color,
    pub col_a: Color,
    pub col_b: Color,
    pub secs_per_flash: f32,
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
    pub fn new(old_color: Color) -> Self {
        Self {
            old_color,
            ..default()
        }
    }

    pub fn bundle(self) -> impl Bundle {
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
