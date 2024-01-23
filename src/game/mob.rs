use crate::prelude::*;

pub mod prelude {
    pub use super::{
        brain::prelude::*, collide::prelude::*, data::prelude::*, effect::prelude::*,
        hp::prelude::*, player::prelude::*, weapon::prelude::*, UNIT,
    };
}

pub const UNIT: f32 = 2.0;

pub mod brain;
pub mod collide;
pub mod data;
pub mod effect;
pub mod hp;
pub mod player;
pub mod weapon;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            collide::Plug,
            effect::Plug,
            hp::Plug,
            player::Plug,
            weapon::Plug,
        ));
    }
}
