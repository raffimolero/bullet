use crate::prelude::*;

pub mod prelude {
    pub use super::Mob;
}

#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Mob {
    Pellet,
    #[default]
    Dart,
}

impl Pack for Mob {
    fn attach(self, commands: &mut Commands, entity: Entity) {
        let mut cmd = commands.entity(entity);
        // two insertion types with respective conversion conventions:
        // - stuff that we always have (self, sprite, hp, ..)
        //   - stuff that we know ahead of time (self, hp, ..) => Type::from(mob)
        //   - stuff that needs game resources (sprite, ..) => mob.method() -> Type
        // - stuff that we might have (hitdamage, weapon)
        //   - stuff that we know ahead of time (weapon, ..) => Type::try_from(mob)
        //   - stuff that needs game resources (no examples yet) => mob.method() -> Option<Type>
        cmd.insert((
            self,
            self.sprite(),
            Vel::default(),
            Hp::from(self),
            DamageTaken::default(),
            HitRadius::from(self),
        ));
        if let Ok(hit_dmg) = HitDamage::try_from(self) {
            cmd.insert(hit_dmg);
        }
        if let Ok(weapon) = Weapon::try_from(self) {
            cmd.insert((weapon, WeaponState::default()));
        }

        use Mob as M;
        match self {
            _ => {}
        };
    }
}

impl Mob {
    pub fn sprite(self) -> SpriteBundle {
        use Mob::*;
        let radius = HitRadius::from(self).0;
        let color = match self {
            Pellet => Color::RED,
            Dart => Color::BLUE,
        };
        Blocc {
            w: radius * 2.0,
            h: radius * 2.0,
            color,
            ..default()
        }
        .bundle()
    }
}

impl From<Mob> for Hp {
    fn from(value: Mob) -> Self {
        use Mob::*;
        Self(match value {
            Pellet => 1,
            Dart => 3,
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

impl TryFrom<Mob> for Weapon {
    type Error = ();

    fn try_from(value: Mob) -> Result<Self, Self::Error> {
        use Mob as M;
        use Weapon as W;
        Ok(match value {
            M::Dart => W::Basic,
            _ => return Err(()),
        })
    }
}

impl TryFrom<Mob> for HitDamage {
    type Error = ();

    fn try_from(value: Mob) -> Result<Self, Self::Error> {
        use Mob::*;
        let dmg = match value {
            Pellet => 1,
            Dart => 1,
        };
        if dmg == 0 {
            Err(())
        } else {
            Ok(Self(dmg))
        }
    }
}
