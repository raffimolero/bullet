use crate::prelude::*;

pub mod prelude {
    pub use super::*;
}

#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Mob {
    Pellet,
    Spore,
    Pod,
    #[default]
    Dart,
    Mosquito,
}

impl Pack for Mob {
    fn attach(self, commands: &mut EntityCommands) {
        // insertion types with respective conversion conventions:
        // - stuff that we always have (self, sprite, hp, ..)
        //   - stuff that never changes (mass, ..) => mob.method() -> Type
        //   - stuff that we know ahead of time (self, hp, ..) => Type::from(mob)
        //   - stuff that needs game resources (sprite, ..) => mob.method(args) -> Type
        // - stuff that we might have (hitdamage, weapon)
        //   - stuff that we know ahead of time (weapon, ..) => Type::try_from(mob)
        //   - stuff that needs game resources (no examples yet) => mob.method(args) -> Option<Type>
        commands.insert((
            self,
            self.sprite(),
            MaxDTf::default(),
            DeltaTf::default(),
            Hp::from(self),
            LastHitBy::default(),
            DamageTaken::default(),
            HitRadius::from(self),
        ));
        if let Ok(hit_dmg) = HitDamage::try_from(self) {
            commands.insert(hit_dmg);
        }
        if let Ok(weapon) = Weapon::try_from(self) {
            commands.attach(weapon);
        }
        if let Ok(brain) = BrainState::try_from(self) {
            commands.attach(brain);
        }

        match self {
            _ => {}
        };
    }
}

impl Mob {
    pub fn sprite(self) -> SpriteBundle {
        let radius = HitRadius::from(self).0;
        let color = match self {
            Mob::Pellet => Color::RED,
            Mob::Spore => Color::YELLOW,
            Mob::Pod => Color::BEIGE,
            Mob::Dart => Color::BLUE,
            Mob::Mosquito => Color::GRAY,
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
        Self(match value {
            Mob::Pellet => 1,
            Mob::Spore => 1,
            Mob::Pod => 5,
            Mob::Dart => 3,
            Mob::Mosquito => 2,
        })
    }
}

impl From<Mob> for HitRadius {
    fn from(value: Mob) -> Self {
        Self(
            UNIT * match value {
                Pellet => 1.0,
                Spore => 1.0,
                Pod => 6.0,
                Dart => 4.0,
                Mosquito => 3.0,
            },
        )
    }
}

impl TryFrom<Mob> for Weapon {
    type Error = ();

    fn try_from(value: Mob) -> Result<Self, Self::Error> {
        use Weapon as W;
        Ok(match value {
            Mob::Dart => W::Basic,
            _ => return Err(()),
        })
    }
}

// TODO: add "hit influence" enum
// imparts force on impact. either outward or in a specific direction.

impl TryFrom<Mob> for HitDamage {
    type Error = ();

    fn try_from(value: Mob) -> Result<Self, Self::Error> {
        let dmg = match value {
            Mob::Pellet => 1,
            Mob::Spore => 1,
            Mob::Pod => 0,
            Mob::Dart => 3,
            Mob::Mosquito => 2,
        };
        if dmg == 0 {
            Err(())
        } else {
            Ok(Self(dmg))
        }
    }
}

impl TryFrom<Mob> for BrainState {
    type Error = ();

    fn try_from(value: Mob) -> Result<Self, Self::Error> {
        match value {
            Mob::Dart | Mob::Mosquito => Ok(Self),
            _ => Err(()),
        }
    }
}
