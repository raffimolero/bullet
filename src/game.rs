use crate::prelude::*;

pub mod prelude {
    pub use super::{
        blocc::prelude::*, level::prelude::*, logic::prelude::*, mob::prelude::*,
        motion::prelude::*, pack::prelude::*, GState, GameLoop,
    };
}

pub mod blocc;
pub mod level;
pub mod logic;
pub mod mob;
pub mod motion;
pub mod pack;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.configure_set(Startup, (Setup::Basic, Setup::Debug).chain())
            .configure_set(
                FixedUpdate,
                (
                    GameLoop::Meta,
                    GameLoop::Control,
                    GameLoop::Ai,
                    GameLoop::Move,
                    GameLoop::CollideCheck,
                    GameLoop::CollideAct,
                )
                    .chain()
                    .run_if(in_state(GState::InGame)),
            )
            .add_plugins((motion::Plug, logic::Plug, level::Plug, mob::Plug));
    }
}

// TODO: configure setup system set

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Setup {
    Basic,
    Debug,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameLoop {
    Meta,
    Control,
    Ai,
    Move,
    CollideCheck,
    CollideAct,
    Effect,
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GState {
    #[default]
    Waiting,
    InGame,
}
