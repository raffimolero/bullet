use crate::prelude::*;

pub mod prelude {
    pub use super::{CommandsExt, Pack};
    pub use bevy::ecs::system::EntityCommands;
}

pub trait Pack {
    fn attach(self, commands: &mut EntityCommands);
}

pub trait CommandsExt {
    fn attach<P: Pack>(&mut self, pack: P) -> &mut Self;
}

impl CommandsExt for EntityCommands<'_, '_, '_> {
    fn attach<P: Pack>(&mut self, pack: P) -> &mut Self {
        pack.attach(self);
        self
    }
}
