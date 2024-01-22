use crate::prelude::*;

pub mod prelude {
    pub use super::{BrainBundle, BrainState, Target};
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, think.in_set(GameLoop::Think));
    }
}

#[derive(Component, Clone, Copy, Default)]
pub struct Target(Option<Entity>);

#[derive(Component, Clone, Copy, Default)]
pub struct DesiredVel(pub Velocity);

#[derive(Component, Clone, Copy, Default)]
pub struct BrainState {
    next_thought: Instant,
}

// TODO: all the read only params.
fn think(mut mobs: Query<(&Mob, &mut BrainState, &mut Target, &mut DesiredVel)>) {
    let now = Instant::now();
    mobs.for_each_mut(|(mob, mut brain_st)| {
        if now > brain_st.next_thought {
            mob.think()
        }
    });
}

#[derive(Bundle)]
pub struct BrainBundle {
    pub target: Target,
    pub weapon_state: WeaponState,
    pub desired_dtf: DesiredVel,
}

fn act_vel(mut mobs: Query<(&DesiredVel, &mut Acceleration)>) {
    mobs.for_each_mut(|(dv, mut v)| {
        v = dv;
    });
}
