use pixels::{Pixels, SurfaceTexture};
use std::fs;
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;
use std::{thread, time};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod level_maker;
use level_maker::*;

mod tile;
use tile::*;

mod screen;
use screen::Screen;

mod collision;
use collision::*;

mod texture;
use texture::Texture;

mod animation;
use animation::*;

mod sprite;
use sprite::*;

mod types;
use types::*;

// Now this main module is just for the run-loop and rules processing.
struct GameState {
    // What data do we need for this game?  Wall positions?
    // Colliders?  Sprites and stuff?
    textures: Vec<Rc<Texture>>,
    sprites: Vec<Sprite>,
    scroll: Vec2i,
    level: u16,
    current_tex: usize,
    shots_left: i32,
    shot_cool_down: i32,
    shot_index: usize,
    frame: usize,
    next_level: bool,
}
// seconds per frame
const DT: f64 = 1.0 / 60.0;

const WIDTH: usize = 240;
const HEIGHT: usize = 240;
const DEPTH: usize = 4;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Anim2D")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
    };
    let mut state = GameState {
        // initial game state...
        sprites: vec![],
        textures: make_menus(),
        scroll: Vec2i(0, 0),
        level: 0,
        current_tex: 0,
        shots_left: 3,
        shot_cool_down: 0,
        shot_index: 0,
        frame: 0,
        next_level: false,
    };
    let tex = Rc::new(Texture::with_file(Path::new("content/space_tileset.png")));
    let tileset = Rc::new(Tileset::new(
        vec![
            Tile { solid: false },
            Tile { solid: false },
            Tile { solid: false },
            Tile { solid: false },
            Tile { solid: false },
            Tile { solid: false },
            Tile { solid: false },
            Tile { solid: false },
        ],
        &tex,
    ));
    let maps = vec![Tilemap::new(Vec2i(0, 0), (64, 6), &tileset, make_map())];
    // How many frames have we simulated?
    let mut frame_count: usize = 0;
    // How many unsimulated frames have we saved up?
    let mut available_time = 0.0;
    // Track end of the last frame
    let mut since = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let mut screen = Screen::wrap(pixels.get_frame(), WIDTH, HEIGHT, DEPTH, Vec2i(0, 0));
            screen.clear(Rgba(255, 255, 255, 255));
            screen.position = state.scroll;

            if state.level == 0 {
                update_menu(&mut state, &input);
                screen.bitblt(
                    &state.textures[state.current_tex], // TODO: JUST MAKE A FUNCTION TO PROVIDE THE TEXTURE
                    Rect {
                        x: 0,
                        y: 0,
                        w: 240,
                        h: 240,
                    },
                    Vec2i(0, 0),
                );
            } else if state.level == 4 {
                state.scroll = Vec2i(0, 0);
                screen.bitblt(
                    &state.textures[5], // TODO: JUST MAKE A FUNCTION TO PROVIDE THE TEXTURE
                    Rect {
                        x: 0,
                        y: 0,
                        w: 240,
                        h: 240,
                    },
                    Vec2i(0, 0),
                );
            } else if state.level == 5 {
                state.scroll = Vec2i(0, 0);
                screen.bitblt(
                    &state.textures[6], // TODO: JUST MAKE A FUNCTION TO PROVIDE THE TEXTURE
                    Rect {
                        x: 0,
                        y: 0,
                        w: 240,
                        h: 240,
                    },
                    Vec2i(0, 0),
                );
                if input.key_held(VirtualKeyCode::Return){
                    state.level = 0;
                }
            } else if state.next_level {
                update_level(&mut state, &input);
                state.scroll = Vec2i(0, 0);
                screen.bitblt(
                    &state.textures[4], // TODO: JUST MAKE A FUNCTION TO PROVIDE THE TEXTURE
                    Rect {
                        x: 0,
                        y: 0,
                        w: 240,
                        h: 240,
                    },
                    Vec2i(0, 0),
                );
            } else {
                for map in maps.iter() {
                    map.draw(&mut screen);
                }
                draw_game(&mut state, &mut screen, frame_count);
            }

            // Flip buffers

            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            // Rendering has used up some time.
            // The renderer "produces" time...
            available_time += since.elapsed().as_secs_f64();
        }

        // Handle input events
        if input.update(event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            // Resize the window if needed
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }
        }
        // And the simulation "consumes" it
        while available_time >= DT {
            // Eat up one frame worth of time
            available_time -= DT;
            if !state.next_level && (state.level == 1 || state.level == 2 || state.level == 3) {
                update_game(&mut state, &input);
                // Increment the frame counter
                frame_count += 1;
                state.frame = frame_count;
            }
        }
        // Request redraw
        window.request_redraw();
        // When did the last frame end?
        since = Instant::now();
    });
}

fn draw_game(state: &mut GameState, screen: &mut Screen, frame_number: usize) {
    for s in state.sprites.iter_mut().rev() {
        s.animate(frame_number);
        screen.draw_sprite(s);
    }
}

fn update_game(state: &mut GameState, input: &WinitInputHelper) {
    // Player control goes here

    let bottom_border = 165;
    if input.key_held(VirtualKeyCode::Up)
        && !(state.sprites[0].position.1 > bottom_border)
        && !(state.sprites[0].position.1 < 0)
    {
        if state.sprites[0].vy > 0.0 {
            state.sprites[0].vy /= 3.0;
        }
        state.sprites[0].vy -= 0.2;
    } else {
        if state.sprites[0].position.1 <= 0 {
            state.sprites[0].position.1 = 0;
        }
        if state.sprites[0].position.1 >= bottom_border {
            state.sprites[0].vy = 0.0;
            state.sprites[0].position.1 = bottom_border;
        } else {
            state.sprites[0].vy += 0.1;
        }
    }
    if state.sprites[0].position.1 >= bottom_border && state.sprites[0].animation.index != 0 {
        state.sprites[0].animation.set_state(0, state.frame)
    }
    if state.sprites[0].position.1 < bottom_border && state.sprites[0].animation.index != 1 {
        state.sprites[0].animation.set_state(1, state.frame)
    }

    if input.key_pressed(VirtualKeyCode::Space) && state.shot_cool_down == 0 {
        handle_shot(state);
    }

    // Scroll & Movement
    if state.level == 1 || state.level == 2 || state.level == 3 {
        // Gather collisions for player
        if player_contacts(&state.sprites) {
            thread::sleep(time::Duration::from_millis(500));
            state.level = 5;
        }

        // Scroll camera
        state.scroll = Vec2i(state.scroll.0 + 2, state.scroll.1);

        // Move Sprite
        state.sprites[0].position.0 += 2;
        state.sprites[0].position.1 += state.sprites[0].vy.min(2.0) as i32;
        let y_pos = state.sprites[0].position.1;
        // Move Hitbox
        for hit_box in &mut state.sprites[0].hit_boxes {
            hit_box.x += 2;
            hit_box.y = y_pos;
        }

        // Move lasers
        for i in 1..4 {
            state.sprites[i].position.0 += 4;
            for hit_box in &mut state.sprites[i].hit_boxes {
                hit_box.x += 4;
            }
            // Move laser out of screen so it doesn't collide with obstacles that haven't loaded yet
            // Once out of the screen update our shot
            if state.sprites[i].position.0 == (state.scroll.0 + 240) {
                state.sprites[i].position.1 = -20;
                state.sprites[i].hit_boxes[0].y = -20;
                state.sprites[i].hit_boxes[0].x = -20;
                if state.shots_left < 3 {
                    state.shots_left += 1;
                    state.sprites[4].animation.index -= 1;
                }
            }
        }
        // Move battery
        state.sprites[4].position.0 += 2;
        state.shot_cool_down = (state.shot_cool_down - 1).max(0);

        laser_contacts(&mut state.sprites, state.frame);
        check_clear(state);
    }
}

// Function that takes care of moving around in the menu
fn update_menu(state: &mut GameState, input: &WinitInputHelper) {
    if input.key_pressed(VirtualKeyCode::Up) && state.current_tex != 3 {
        if state.current_tex != 0 {
            state.current_tex -= 1;
            state.current_tex %= 3;
        } else {
            state.current_tex = 2;
        }
    }
    if input.key_pressed(VirtualKeyCode::Down) && state.current_tex != 3 {
        state.current_tex += 1;
        state.current_tex %= 3;
    }
    if input.key_pressed(VirtualKeyCode::Return) {
        if state.current_tex == 0 {
            state.level = 1;
            state.sprites = level_1();
            for s in state.sprites.iter_mut() {
                s.animation.set_state(0, state.frame)
            }
        } else if state.current_tex == 1 {
            load_game(state);
        } else if state.current_tex == 2 {
            state.current_tex = 3;
        } else if state.current_tex == 3 {
            state.current_tex = 0;
        }
    }
}

// Shooting mechanics
fn handle_shot(state: &mut GameState) {
    // If we have a shot, shoot
    if state.shots_left > 0 {
        state.sprites[4].animation.index += 1; // Update battery
        let i = (state.shot_index % 3) + 1;
        state.sprites[i].position = state.sprites[0].position;
        state.sprites[i].position.0 += 30;
        state.sprites[i].position.1 += 20;
        let x_pos = state.sprites[i].position.0;
        let y_pos = state.sprites[i].position.1;
        for hit_box in &mut state.sprites[i].hit_boxes {
            hit_box.x = x_pos;
            hit_box.y = y_pos;
        }
        state.shots_left -= 1;
        state.shot_cool_down = 20;
        state.shot_index = state.shot_index + 1;
    }
}

// Check if player has cleared the level
#[allow(unused_must_use)]
fn check_clear(state: &mut GameState) {
    if state.sprites[0].position.0 >= 2500 {
        thread::sleep(time::Duration::from_millis(1000));
        if state.level != 3 {
            let save = state.level.to_string();
            fs::write("content/save.txt", save);
            state.next_level = true;
        } else {
            state.level = 4;
        }
    }
}

fn update_level(state: &mut GameState, input: &WinitInputHelper) {
    // TODO: SAVE GAME
    if input.key_pressed(VirtualKeyCode::Return) {
        state.next_level = false;
        state.level += 1;
        state.shots_left = 3;
        if state.level == 2 {
            state.sprites = level_2();
        } else if state.level == 3 {
            state.sprites = level_3();
        }
        for s in state.sprites.iter_mut() {
            s.animation.set_state(0, state.frame)
        }
    }
}

fn load_game(state: &mut GameState) {
    if let Ok(level) = fs::read_to_string("content/save.txt") {
        if level == "0" {
            state.level = 1;
            state.sprites = level_1();
            for s in state.sprites.iter_mut() {
                s.animation.set_state(0, state.frame)
            }
        } else if level == "1" {
            state.next_level = true;
            state.level = 1;
        } else if level == "2" {
            state.next_level = true;
            state.level = 2;
        }
    }
}
