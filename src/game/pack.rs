use crate::prelude::*;

pub mod prelude {
    pub use super::Pack;
}

pub trait Pack {
    fn attach(self, commands: &mut Commands, entity: Entity);
}
