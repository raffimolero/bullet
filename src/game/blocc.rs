use bevy::prelude::*;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

pub struct Blocc {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub h: f32,
    pub color: Color,
}

impl Default for Blocc {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 50.0,
            h: 50.0,
            color: Color::ALICE_BLUE,
        }
    }
}

impl Blocc {
    pub fn bundle(self) -> SpriteBundle {
        let Self {
            x,
            y,
            z,
            w,
            h,
            color,
        } = self;

        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(w, h)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, z),
            ..default()
        }
    }
}

pub fn setup() {}
