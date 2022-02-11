use derefable::Derefable;
use enumflags2::{make_bitflags, bitflags, BitFlags};
use macroquad::prelude::*;

#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EntityFlags {
    Player           = 1 << 0,
    Enemy            = 1 << 1,
    Bullet           = 1 << 2,
    Smart            = 1 << 3,
    DespawnOffScreen = 1 << 4,
}

#[derive(Clone, Default, Debug)]
pub struct Weapon {
    pub ammo: u32,
}

#[derive(Clone, Default, Debug)]
pub struct Arsenal {
    pub bullet_shooter: Option<Weapon>,
}

#[derive(Clone, Derefable, Debug)]
pub struct Entity {
    #[deref(mutable)]
    pub bbox: Rect,
    pub texture: Texture2D,
    pub speed: f32,
    pub arsenal: Arsenal,
    pub weapon_cooldown: f32, // Only one weapon allowed at a time, so we can manage that
    pub rotation: f32, // in radians
    pub flags: BitFlags<EntityFlags>,
}

#[allow(dead_code)]
impl Entity {
    pub fn is_bullet(&self) -> bool {
        self.flags.intersects(EntityFlags::Bullet)
    }
    pub fn is_player_bullet(&self) -> bool {
        self.flags.intersects(make_bitflags!(EntityFlags::{Player | Bullet}))
    }
    pub fn is_enemy_bullet(&self) -> bool {
        self.flags.intersects(make_bitflags!(EntityFlags::{Enemy | Bullet}))
    }
    pub fn is_player(&self) -> bool {
        self.flags.intersects(EntityFlags::Player) && !self.flags.intersects(EntityFlags::Bullet)
    }
    pub fn is_enemy(&self) -> bool {
        self.flags.intersects(EntityFlags::Enemy) && !self.flags.intersects(EntityFlags::Bullet)
    }
    pub fn should_despawn_off_screen(&self) -> bool {
        self.flags.intersects(EntityFlags::DespawnOffScreen)
    }
    pub fn pos(&self) -> Vec2 {
        vec2(self.x, self.y)
    }
}
