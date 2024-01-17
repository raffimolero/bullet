use crate::prelude::*;

pub mod prelude {
    pub use super::Pack;
}

// TODO: CommandsExt

pub trait Pack {
    fn attach(self, commands: &mut Commands, entity: Entity);
}
