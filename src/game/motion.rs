use crate::prelude::*;

pub mod prelude {
    pub use super::{Acceleration, Friction, MaxAccel, Velocity};
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_schedule.add_systems(
            Update,
            (
                cap_acceleration,
                acceleration,
                /*cap_motion,*/ motion,
                friction,
            )
                .chain()
                // TODO: fix systemset
                .in_set(Motion),
        );
    }
}

/*
/// clamps a DeltaTf.
#[derive(Component, Clone, Copy)]
pub struct MaxDTf {
    pub speed: f32,
    pub rotation: f32,
    pub growth: f32,
}

impl Default for MaxDTf {
    fn default() -> Self {
        Self {
            speed: UNIT * 4.0,
            rotation: TAU,
            growth: 2.0,
        }
    }
}

impl MaxDTf {
    fn clamp(self, delta: &mut Velocity) {
        let Self {
            speed,
            rotation,
            growth,
        } = self;
        let length_squared = delta.velocity.length_squared();
        if length_squared > speed * speed {
            delta.velocity *= speed / length_squared.sqrt();
        }
        delta.rotation = delta.rotation.clamp(-rotation, rotation);
        delta.growth = delta.growth.clamp(self.growth.recip(), self.growth);
    }
}

fn cap_motion(mut movers: Query<(&MaxDTf, &mut Velocity)>) {
    movers.for_each_mut(|(max, dtf)| {
        max.clamp(dtf.as_mut());
    });
}
*/

#[derive(Component, Clone, Copy, PartialEq)]
pub struct MaxAccel {
    /// omnidirectional.
    pub speed: f32,
    /// bidirectional.
    pub rotation: f32,
    /// bidirectional.
    pub growth: f32,
}

impl MaxAccel {
    fn clamp(self, accel: &mut Acceleration) {
        let Self {
            speed,
            rotation,
            growth,
        } = self;
        let length_squared = accel.velocity.length_squared();
        if length_squared > speed * speed {
            accel.velocity *= speed / length_squared.sqrt();
        }
        accel.rotation = accel.rotation.clamp(-rotation, rotation);
        accel.growth = accel.growth.clamp(1.0 / self.growth, self.growth);
    }
}

fn cap_acceleration(mut movers: Query<(&MaxAccel, &mut Acceleration)>) {
    movers.for_each_mut(|(max, mut accel)| {
        max.clamp(&mut accel);
    });
}

#[derive(Component, Clone, Copy, PartialEq, Default)]
pub struct Acceleration {
    /// additive.
    pub velocity: Vec2,
    /// additive.
    pub rotation: f32,
    /// additive.
    pub growth: f32,
}

impl MulAssign<f32> for Acceleration {
    fn mul_assign(&mut self, rhs: f32) {
        self.velocity *= rhs;
        self.rotation *= rhs;
        self.growth *= rhs;
    }
}

impl Mul<f32> for Acceleration {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl AddAssign<Acceleration> for Velocity {
    fn add_assign(&mut self, rhs: Acceleration) {
        self.velocity += rhs.velocity;
        self.rotation += rhs.rotation;
        self.growth += rhs.growth;
    }
}

impl Add<Acceleration> for Velocity {
    type Output = Self;

    fn add(mut self, rhs: Acceleration) -> Self::Output {
        self *= rhs;
        self
    }
}

fn acceleration(time: Res<Time>, mut movers: Query<(&Acceleration, &mut Velocity)>) {
    let delta = time.delta_seconds();
    movers.for_each_mut(|(accel, mut vel)| {
        vel += accel * delta;
    });
}

#[derive(Component, Clone, Copy, PartialEq)]
pub struct Velocity {
    /// additive.
    pub velocity: Vec2,
    /// additive.
    pub rotation: f32,
    /// multiplicative.
    pub growth: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            velocity: Vec2::ZERO,
            rotation: 0.0,
            growth: 1.0,
        }
    }
}

impl MulAssign<f32> for Velocity {
    fn mul_assign(&mut self, rhs: f32) {
        self.velocity *= rhs;
        self.rotation *= rhs;
        self.growth = self.growth.powf(rhs);
    }
}

impl Mul<f32> for Velocity {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<Velocity> for Transform {
    fn mul_assign(&mut self, rhs: Velocity) {
        self.translation += rhs.velocity.extend(0.0);
        let (y, x, z) = self.rotation.to_euler(EulerRot::default());
        self.rotation = Quat::from_euler(EulerRot::default(), y, x, z + rhs.rotation);
        self.scale *= rhs.growth;
    }
}

impl Mul<Velocity> for Transform {
    type Output = Self;

    fn mul(mut self, rhs: Velocity) -> Self::Output {
        self *= rhs;
        self
    }
}

pub fn motion(time: Res<Time>, mut movers: Query<(&Transform, &mut Transform)>) {
    let delta = time.delta_seconds();
    movers.for_each_mut(|(dtf, mut tf)| {
        *tf *= dtf * delta;
    });
}

#[derive(Component, Clone, Copy)]
pub struct Friction {
    /// multiplicative.
    pub speed: f32,
    /// multiplicative.
    pub rotation: f32,
    /// multiplicative.
    pub growth: f32,
}

impl Default for Friction {
    fn default() -> Self {
        const FACTOR: f32 = 1.0 - 1.0 / (1 >> 8) as f32;
        Self {
            speed: FACTOR,
            rotation: FACTOR,
            growth: FACTOR,
        }
    }
}

impl MulAssign<f32> for Friction {
    fn mul_assign(&mut self, rhs: f32) {
        self.speed = self.speed.powf(rhs);
        self.rotation = self.rotation.powf(rhs);
        self.growth = self.growth.powf(rhs);
    }
}

impl Mul<f32> for Friction {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<Friction> for Velocity {
    fn mul_assign(&mut self, rhs: Friction) {
        self.velocity *= rhs.speed;
        self.rotation *= rhs.rotation;
        self.growth *= rhs.growth;
    }
}

impl Mul<Friction> for Velocity {
    type Output = Self;

    fn mul(mut self, rhs: Friction) -> Self::Output {
        self *= rhs;
        self
    }
}

fn friction(time: Res<Time>, mut movers: Query<(&Friction, &mut Velocity)>) {
    let delta = time.delta_seconds();
    movers.for_each_mut(|(fric, mut vel)| {
        vel *= fric * delta;
    });
}
