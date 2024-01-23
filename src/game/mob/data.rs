use crate::prelude::*;

pub mod prelude {
    pub use super::*;
}

// TODO: static objects should be bouncy. Negative friction?
// all mobs have acceleration and velocity right
#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Mob {
    Pellet,
    Spore,
    Pod,
    #[default]
    Dart,
    Mosquito,
}

// TODO: MobData struct so we can just specify all the data per mob at once

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
            Velocity::default(),
            Acceleration::default(),
            Hp::from(self),
            LastHitBy::default(),
            DamageTaken::default(),
        ));
        if let Ok(hr) = HitRadius::try_from(self) {
            commands.insert(hr);
        }
        if let Ok(max_accel) = MaxAccel::try_from(self) {
            commands.insert(max_accel);
        }
        if let Ok(fric) = Friction::try_from(self) {
            commands.insert(fric);
        }
        if let Ok(hit_dmg) = HitDamage::try_from(self) {
            commands.insert(hit_dmg);
        }
        if let Ok(weapon) = Weapon::try_from(self) {
            commands.attach(weapon);
        }
        if let Ok(brain) = BrainState::try_from(self) {
            commands.insert(brain);
        }

        // other special stuff maybe idk
        match self {
            _ => {}
        };
    }
}

impl Mob {
    pub fn sprite(self) -> SpriteBundle {
        let (color, radius) = match self {
            Mob::Pellet => (Color::RED, 1.0),
            Mob::Spore => (Color::YELLOW, 1.0),
            Mob::Pod => (Color::BEIGE, 6.0),
            Mob::Dart => (Color::BLUE, 4.0),
            Mob::Mosquito => (Color::GRAY, 3.0),
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

impl TryFrom<Mob> for HitRadius {
    type Error = ();

    fn try_from(value: Mob) -> Result<Self, Self::Error> {
        Ok(Self(
            UNIT * match value {
                Mob::Pellet => 1.0,
                Mob::Spore => 1.0,
                Mob::Pod => 6.0,
                Mob::Dart => 4.0,
                Mob::Mosquito => 3.0,
                _ => return Err(()),
            },
        ))
    }
}

impl TryFrom<Mob> for MaxAccel {
    type Error = ();

    fn try_from(value: Mob) -> Result<Self, Self::Error> {
        let base = MaxAccel::default();
        let none = Err(());
        let light = Ok(base * 0.5);
        let medium = Ok(base);
        let heavy = Ok(base * 2.0);
        match value {
            Mob::Pellet => none,
            Mob::Spore => light,
            Mob::Pod => heavy,
            Mob::Dart => medium,
            Mob::Mosquito => light,
        }
    }
}

impl TryFrom<Mob> for Friction {
    type Error = ();

    fn try_from(value: Mob) -> Result<Self, Self::Error> {
        let base = Friction::default();
        let ice = Err(());
        let smooth = Ok(base * 0.5);
        let matte = Ok(base);
        let rough = Ok(base * 2.0);
        match value {
            Mob::Pellet => ice,
            Mob::Spore => smooth,
            Mob::Pod => rough,
            Mob::Dart => matte,
            Mob::Mosquito => smooth,
        }
    }
}

impl TryFrom<Mob> for Weapon {
    type Error = ();

    fn try_from(value: Mob) -> Result<Self, Self::Error> {
        use Weapon as W;
        Ok(match value {
            Mob::Mosquito => W::Basic,
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
            Mob::Dart | Mob::Mosquito => Ok(Self::default()),
            _ => Err(()),
        }
    }
}
