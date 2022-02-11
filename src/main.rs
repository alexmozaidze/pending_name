#![forbid(unsafe_code)]
#![feature(map_first_last, let_else)]

mod consts;
mod debug;
mod entity;
mod player;

use consts::*;
use debug::DebugInfo;
use entity::{Entity, EntityFlags, Arsenal, Weapon};
use player::PlayerAction;

use enum_map::{enum_map, EnumMap};
use enumflags2::make_bitflags;
use macroquad::{input::KeyCode as Key, prelude::*};
use macroquad_canvas::Canvas2D;
use slotmap::{DefaultKey as EntityKey, SlotMap};
use std::collections::BTreeMap;
use maplit::btreemap;

#[derive(Debug)]
struct Textures {
    player: Texture2D,
    bullet: Texture2D,
}

macro_rules! load_texture {
    ($path:expr) => {
        Texture2D::from_file_with_format(
            DATA.get_file($path).unwrap().contents(),
            Some(ImageFormat::Png),
        )
    };
}

// Function controlling the mouse
// NOTE: Currently just returns mouse position back. I'll change it if I decide to make
// some mechanic changes to the mouse, for now it's a dummy function.
#[allow(unused_variables)]
fn mouse_to_aim(player: &Entity, mouse_x: f32, mouse_y: f32) -> (f32, f32) {
    let aim_x: f32 = mouse_x;
    let aim_y: f32 = mouse_y;

    (aim_x, aim_y)
}

#[macroquad::main("pending_name")]
async fn main() {
    let canvas = Canvas2D::new(WIDTH, HEIGHT);

    let controls: EnumMap<PlayerAction, [Option<Key>; 2]> = enum_map! {
        PlayerAction::MoveUp    => [ Some(Key::Up),    Some(Key::W) ],
        PlayerAction::MoveLeft  => [ Some(Key::Left),  Some(Key::A) ],
        PlayerAction::MoveDown  => [ Some(Key::Down),  Some(Key::S) ],
        PlayerAction::MoveRight => [ Some(Key::Right), Some(Key::D) ],
        PlayerAction::Dash      => [ Some(Key::Z),     Some(Key::H) ],
        PlayerAction::Attack    => [ Some(Key::X),     Some(Key::J) ],
    };
    let action_func = |player_action: PlayerAction, f: fn(Key) -> bool| -> bool {
        controls[player_action]
            .iter()
            .map(|x| x.map(f))
            .any(|x| x == Some(true))
    };
    // Helper functions to call specific event action
    #[allow(unused_variables)]
    let (action_pressed, action_released, action_down) = (
        |player_action| action_func(player_action, is_key_pressed),
        |player_action| action_func(player_action, is_key_released),
        |player_action| action_func(player_action, is_key_down),
    );

    let mut entities: SlotMap<EntityKey, Entity> = SlotMap::new();

    let textures = Textures {
        player: load_texture!("ship_L.png"),
        bullet: load_texture!("ship_B.png"),
    };

    let player_key = entities.insert(Entity {
        bbox: Rect {
            x: PLAYER_X,
            y: PLAYER_Y,
            w: PLAYER_WIDTH,
            h: PLAYER_HEIGHT,
        },
        texture: textures.player,
        speed: PLAYER_BASE_SPEED,
        arsenal: Arsenal {
            bullet_shooter: Some(Weapon { ammo: 9000 }),
        },
        weapon_cooldown: 0.,
        rotation: 0.,
        flags: make_bitflags!(EntityFlags::{Player}),
    });

    // I'm too lazy to type this out every time
    macro_rules! player {
        () => {
            entities[player_key]
        };
    }

    let mut debug_info: EnumMap<DebugInfo, Box<str>> = EnumMap::default();

    macro_rules! debug_set {
        ($index:expr, $value:expr $(,)?) => {{
            debug_info[$index] = $value.to_string().into_boxed_str();
        }};
    }

    let mut fps_counter: u32 = 0;

    debug_set!(DebugInfo::Fps, format!("Fps: {}", get_fps()));
    #[cfg(not(debug_assertions))]
    debug_set!(DebugInfo::DebugModeNotice, EXIT_DEBUG_MODE_MESSAGE);

    // Used for logic loop
    let mut time_last = get_time();

    let mut debug_mode = true;

    let demo_level: BTreeMap<i64, Entity> = btreemap!{
        5 => Entity {
            bbox: Rect {
                x: 0.,
                y: 0.,
                w: PLAYER_BULLET_WIDTH,
                h: PLAYER_BULLET_HEIGHT,
            },
            texture: textures.bullet,
            speed: PLAYER_BULLET_SPEED,
            arsenal: Arsenal::default(),
            weapon_cooldown: 0.,
            rotation: 0.,
            flags: make_bitflags!(EntityFlags::{Enemy}),
        },
    };

    // TODO: Implement enemy spawning system
    let level_start_timestamp: f64 = get_time();

    loop {
        #[cfg(feature = "target_fps")]
        {
            const TARGET_FPS: f32 = 10.;
            const MINIMUM_FRAME_TIME: f32 = 1. / TARGET_FPS;

            let frame_time = get_frame_time();

            if frame_time < MINIMUM_FRAME_TIME {
                let time_to_sleep = (MINIMUM_FRAME_TIME - frame_time) * 1000.;
                std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
            }
        }

        // Update FPS info every 15 ticks
        fps_counter += 1;
        if fps_counter == 15 {
            fps_counter = 0;
            debug_set!(DebugInfo::Fps, format!("Fps: {}", get_fps()));
        }

        let (mouse_x, mouse_y) = canvas.mouse_position();
        let (aim_x, aim_y) = mouse_to_aim(&player![], mouse_x, mouse_y);

        let dt: f32 = (get_time() - time_last) as f32 * UPDATE_TIME;
        time_last = get_time();

        // Player Controls
        let mut player_step = Vec2::ZERO;

        if action_down(PlayerAction::MoveLeft) {
            player_step.x = -1.;
        }
        if action_down(PlayerAction::MoveRight) {
            player_step.x = 1.;
        }
        if action_down(PlayerAction::MoveUp) {
            player_step.y = -1.;
        }
        if action_down(PlayerAction::MoveDown) {
            player_step.y = 1.;
        }

        // Clamp step values and move the player
        if let Some(step) = player_step.try_normalize() {
            player_step = step * player![].speed * dt;
            player![].bbox = player![].offset(player_step);
        }

        // Prevent player from going off-screen
        player![].x = player![].x.clamp(0., WIDTH - player![].w);
        player![].y = player![].y.clamp(0., HEIGHT - player![].h);

        debug_set!(
            DebugInfo::Player,
            format!(
                "Player {{ \
                    x: {x:.2}, \
                    y: {y:.2}, \
                    w: {w}, \
                    h: {h}, \
                    speed: {speed}, \
                    step: [{step_x:.2}, {step_y:.2}] \
                }}",
                x = player![].x,
                y = player![].y,
                w = player![].w,
                h = player![].h,
                speed = player![].speed,
                step_x = player_step.x,
                step_y = player_step.y,
            )
        );

        // Helper macro to spawn a bullet
        macro_rules! spawn_bullet {
            () => {
                {
                    entities.insert(Entity {
                        bbox: Rect {
                            x: player![].x + (player![].w / 2.) - (PLAYER_BULLET_WIDTH / 2.),
                            y: player![].y + (player![].h / 2.) - (PLAYER_BULLET_HEIGHT / 2.),
                            w: PLAYER_BULLET_WIDTH,
                            h: PLAYER_BULLET_HEIGHT,
                        },
                        texture: textures.bullet,
                        speed: PLAYER_BULLET_SPEED,
                        arsenal: Arsenal::default(),
                        weapon_cooldown: 0.,
                        rotation: f32::atan2(
                            aim_x - player![].x - (PLAYER_BULLET_HEIGHT / 2.),
                            -(aim_y - player![].y - (PLAYER_BULLET_WIDTH / 2.)),
                        ),
                        flags: make_bitflags!(EntityFlags::{Player | Bullet | DespawnOffScreen}),
                    });
                }
            }
        }

        // NOTE
        // 3 ways to shoot:
        // LMB - Normal shot
        // RMB - Machine gun
        // Space - Power shot
        if is_key_down(Key::Space) || is_mouse_button_pressed(MouseButton::Left) {
            spawn_bullet!();
        }

        // Tick entities
        entities.retain(|_, entity| {
            #[allow(clippy::if_same_then_else)] // TODO: Remove this obnoxious tag
            if entity.is_bullet() {
                let step: Vec2 =
                    vec2(entity.rotation.sin(), -entity.rotation.cos()) * entity.speed * dt;

                entity.bbox = entity.offset(step);
            } else if entity.is_enemy() {
                // TODO: Tick enemy weapon cooldown
            } else if entity.is_player() {
                // TODO: Tick player weapon cooldown
            }

            if entity.should_despawn_off_screen() {
                !(entity.x > WIDTH
                    || entity.x < -entity.w
                    || entity.y > HEIGHT
                    || entity.y < -entity.h)
            } else {
                true
            }
        });

        debug_set!(DebugInfo::Entities, format!("Entities: {}", entities.len()));

        // >>> DRAWING <<<
        set_camera(&canvas.camera);

        clear_background(LIGHTGRAY);

        // A circle LMAO
        draw_circle_lines(aim_x, aim_y, 20., 2., BLACK);

        // Drawing entities
        for entity in entities.values() {
            draw_texture_ex(
                entity.texture,
                ((entity.x + (entity.w / 2.)) - (entity.texture.width() / 2.)).floor(),
                ((entity.y + (entity.h / 2.)) - (entity.texture.height() / 2.)).floor(),
                WHITE,
                DrawTextureParams {
                    rotation: entity.rotation,
                    ..Default::default()
                },
            );

            if debug_mode {
                draw_rectangle(
                    entity.x,
                    entity.y,
                    entity.w,
                    entity.h,
                    Color::from_rgba(0, 255, 0, 90),
                );
            }
        }

        set_default_camera();

        clear_background(BLACK);

        canvas.draw();

        // Toggle debug mode
        if is_key_down(Key::RightControl) && is_key_pressed(Key::F12) {
            debug_mode = !debug_mode;
        }

        // Draw debug info
        if debug_mode {
            for (i, (key, debug_message)) in debug_info
                .iter()
                .filter(|(_, s)| !s.trim().is_empty())
                .enumerate()
            {
                let color: Color = if key == DebugInfo::DebugModeNotice {
                    RED
                } else {
                    GRAY
                };

                draw_text(
                    debug_message,
                    0.,
                    DEBUG_FONT_SIZE * (i as f32 + 1.),
                    DEBUG_FONT_SIZE,
                    color,
                );
            }
        }

        next_frame().await;
    }
}
