use crate::game::prelude::*;

use bevy::prelude::*;

pub mod prelude {
    pub use super::Mob;
}

#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Mob {
    Pellet,
    #[default]
    Dart,
}

impl Mob {
    pub fn attach(self, commands: &mut Commands, entity: Entity) {
        let mut cmd = commands.entity(entity);
        cmd.insert((
            self,
            Hp::from(self),
            HitRadius::from(self),
            HitDamage::from(self),
        ));
        if let Some(weapon) = self.weapon() {
            cmd.insert((weapon, WeaponState::default()));
        }

        use Mob as M;
        match self {
            _ => {}
        };
    }

    pub fn weapon(self) -> Option<Weapon> {
        use Mob as M;
        use Weapon as W;
        Some(match self {
            M::Dart => W::Basic,
            _ => return None,
        })
    }
}

impl From<Mob> for Hp {
    fn from(value: Mob) -> Self {
        use Mob::*;
        Self(match value {
            Pellet => 1,
            Dart => 10,
        })
    }
}

impl From<Mob> for HitRadius {
    fn from(value: Mob) -> Self {
        use Mob::*;
        Self(
            UNIT * match value {
                Pellet => 1.0,
                Dart => 3.0,
            },
        )
    }
}

impl From<Mob> for HitDamage {
    fn from(value: Mob) -> Self {
        use Mob::*;
        Self(match value {
            Pellet => 1,
            Dart => 1,
        })
    }
}
