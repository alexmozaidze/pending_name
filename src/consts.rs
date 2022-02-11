// NOTE: This file is ignored by rustfmt, keep things pretty ;)

use include_dir::{include_dir, Dir};

pub static DATA: Dir = include_dir!("data");

#[cfg(not(debug_assertions))]
pub const EXIT_DEBUG_MODE_MESSAGE: &str = "Press RCtrl+F12 to exit debug mode";
pub const DEBUG_FONT_SIZE: f32 = 22.;

//pub const FONT_SIZE: f32 = 18.;

pub const UPDATE_TIME:  f32 = 60_f32;

pub const WIDTH:  f32 = 800_f32;
pub const HEIGHT: f32 = 600_f32;

pub const PLAYER_BASE_SPEED:    f32 = 6.;
pub const PLAYER_HEIGHT:        f32 = 32.;
pub const PLAYER_WIDTH:         f32 = 32.;
pub const PLAYER_X:             f32 = WIDTH / 2.;
pub const PLAYER_Y:             f32 = HEIGHT - PLAYER_HEIGHT;
pub const PLAYER_BULLET_WIDTH:  f32 = 30.;
pub const PLAYER_BULLET_HEIGHT: f32 = 30.;
pub const PLAYER_BULLET_SPEED:  f32 = 8.;
