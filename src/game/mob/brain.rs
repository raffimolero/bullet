use crate::prelude::*;

pub mod prelude {
    pub use super::{BrainBundle, BrainState, Target};
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_schedule(
            Think
                .run_if(in_state(GState::InGame).before(Motion))
                .add_systems(Update, think.in_set(Think)),
        );
    }
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Think;

#[derive(Component, Clone, Copy, Default)]
pub struct Target(Option<Entity>);

#[derive(Component, Clone, Copy, Default)]
pub struct DesiredDTf(pub DeltaTf);

#[derive(Component, Clone, Copy, Default)]
pub struct BrainState {
    next_thought: Instant,
}

fn think(mut mobs: Query<(&Mob, &mut BrainState)>) {
    let now = Instant::now();
    mobs.for_each_mut(|(mob, mut brain_st)| {
        if now > brain_st.next_thought {
            mob
        }
    });
}

#[derive(Bundle)]
pub struct BrainBundle {
    pub target: Target,
    pub weapon_state: WeaponState,
    pub desired_dtf: DesiredDTf,
}

fn act_dtf(mut mobs: Query<(&DesiredDTf, Option<&mut DeltaTf>)>) {
    mobs.for_each_mut(|(dv, v)| {
        if let Some(mut v) = v {
            v = dv;
        }
    });
}
