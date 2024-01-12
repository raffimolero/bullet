use super::prelude::*;

use bevy::prelude::*;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, lifespan.run_if(in_state(GState::InGame)));
    }
}

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Lifespan(pub Instant);

fn lifespan(mut commands: Commands, objects: Query<(Entity, &Lifespan)>) {
    let now = Instant::now();
    objects.for_each(|(entity, lifespan)| {
        if now > lifespan.0 {
            commands.entity(entity).despawn_recursive();
        }
    });
}
