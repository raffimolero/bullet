/*!
    Brain components:
Acceleration
WeaponState
Target
*/

use crate::prelude::*;

pub mod prelude {
    pub use super::{BrainState, Target};
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, think.in_set(GameLoop::Ai));
    }
}

#[derive(Component, Clone, Copy, Default)]
pub struct Target(Option<Entity>);

pub struct TargetInfo {
    position: GlobalTransform,
    velocity: Velocity,
}

#[derive(Component, Clone, Copy)]
pub struct BrainState {
    next_thought: Instant,
}

impl Default for BrainState {
    fn default() -> Self {
        Self {
            next_thought: Instant::now(),
        }
    }
}

fn think(
    mut mobs: Query<(
        &Mob,
        &mut BrainState,
        &GlobalTransform,
        &Velocity,
        Option<&mut Acceleration>,
        Option<&mut WeaponState>,
        Option<&mut Target>,
    )>,
    targets: Query<(&GlobalTransform, Option<&Velocity>)>,
) {
    let now = Instant::now();
    mobs.for_each_mut(|(mob, mut brain_st, gtf, vel, accel, wpn_st, mut target)| {
        if now > brain_st.next_thought {
            brain_st.next_thought = now
                + mob
                    .retarget(
                        gtf,
                        vel,
                        accel.opt_ref(),
                        wpn_st.opt_ref(),
                        target.opt_mut(),
                        &targets,
                    )
                    .0;
        }
        mob.think(gtf, vel, accel, wpn_st, target.opt_ref(), &targets);
    });
}

struct Cooldown(Duration);

impl Mob {
    fn retarget(
        self,
        pos: &GlobalTransform,
        vel: &Velocity,
        accel: Option<&Acceleration>,
        wpn_st: Option<&WeaponState>,
        target: Option<Mut<Target>>,
        targets: &Query<(&GlobalTransform, Option<&Velocity>)>,
    ) -> Cooldown {
        // NOTE: global transform and transform are different
        // one could easily attach a dart to a rotated parent and its ai would just break
        let (scl, rot, tl) = pos.to_scale_rotation_translation();
        let target_info = target
            .as_ref()
            .and_then(|target| target.0)
            .and_then(|target| targets.get(target).ok())
            .map(|(gtf, vel)| TargetInfo {
                position: *gtf,
                velocity: vel.copied().unwrap_or_default(),
            });

        match (self, target_info) {
            (Mob::Dart, Some(_)) => {} // nothing. darts will never retarget.
            (Mob::Dart, None) => todo!("find target"),
            (
                Mob::Mosquito,
                Some(TargetInfo {
                    position: t_pos,
                    velocity: t_vel,
                }),
            ) => {
                todo!("retarget when too far.")
            }
            (Mob::Mosquito, None) => todo!("find target"),
            _ => {}
        }

        Cooldown(Duration::from_millis(200))
    }

    fn think(
        self,
        pos: &GlobalTransform,
        vel: &Velocity,
        accel: Option<Mut<Acceleration>>,
        wpn_st: Option<Mut<WeaponState>>,
        target: Option<&Target>,
        targets: &Query<(&GlobalTransform, Option<&Velocity>)>,
    ) {
        // NOTE: global transform and transform are different
        // one could easily attach a dart to a rotated parent and its ai would just break
        let (scl, rot, tl) = pos.to_scale_rotation_translation();

        let target_info = target
            .as_ref()
            .and_then(|target| target.0)
            .and_then(|target| targets.get(target).ok())
            .map(|(gtf, vel)| TargetInfo {
                position: *gtf,
                velocity: vel.copied().unwrap_or_default(),
            });

        match (self, target_info) {
            (
                Mob::Dart,
                Some(TargetInfo {
                    position: t_pos,
                    velocity: t_vel,
                }),
            ) => {
                todo!("face and accelerate towards target")
            }
            (Mob::Dart, None) => {}
            (
                Mob::Mosquito,
                Some(TargetInfo {
                    position: t_pos,
                    velocity: t_vel,
                }),
            ) => {
                todo!("face target and shoot, move side to side")
            }
            _ => {}
        }
    }
}

// fn act_vel(mut mobs: Query<(&DesiredVel, &mut Acceleration)>) {
//     mobs.for_each_mut(|(dv, mut v)| {
//         v = dv;
//     });
// }
