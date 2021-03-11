use pixels::{Pixels, SurfaceTexture};
use std::path::Path;
use std::{thread, time};
use std::rc::Rc;
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use rand::{thread_rng, Rng};

mod tile;
use tile::*;
// Whoa what's this?
// Mod without brackets looks for a nearby file.
mod screen;
// Then we can use as usual.  The screen module will have drawing utilities.
use screen::Screen;
// Collision will have our collision bodies and contact types
mod collision;
// Lazy glob imports
use collision::*;
// Texture has our image loading and processing stuff
mod texture;
use texture::Texture;
// Animation will define our animation datatypes and blending or whatever
mod animation;
use animation::*;
// Sprite will define our movable sprites
mod sprite;
// Lazy glob import, see the extension trait business later for why
use sprite::*;
// And we'll put our general purpose types like color and geometry here:
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
    current_tex : usize,
    shots_left: i32,
    shot_cool_down: i32,
    shot_index: usize,
    frame: usize,
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
    let llama = Rc::new(Texture::with_file(Path::new("content/llama.png")));
    //let king = Rc::new(Texture::with_file(Path::new("content/king.png")));

    let asteroid = Rc::new(Texture::with_file(Path::new("content/asteroid.png")));
    let break_asteroid = Rc::new(Texture::with_file(Path::new("content/b_asteroid.png")));  
    let laser =  Rc::new(Texture::with_file(Path::new("content/laser.png")));
    let battery = Rc::new(Texture::with_file(Path::new("content/battery.png")));
    let ship = Rc::new(Texture::with_file(Path::new("content/ship.png")));

    let menu_1 = Rc::new(Texture::with_file(Path::new("content/screens/play.png")));
    let menu_2 = Rc::new(Texture::with_file(Path::new("content/screens/load.png")));
    let game_over = Rc::new(Texture::with_file(Path::new("content/screens/game_over.png")));
    let mut state = GameState {
        // initial game state...
        sprites: vec![Sprite::new(
            &llama,
            Animation::new(vec![ AnimationState {
                frames: vec![Rect {
                    x: 0,
                    y: 0,
                    w: 48,
                    h: 48,
                }, Rect {
                    x: 0,
                    y: 48,
                    w: 48,
                    h: 48,
                }, Rect {
                    x: 0,
                    y: 96,
                    w: 48,
                    h: 48,
                },
                Rect {
                    x: 0,
                    y: 144,
                    w: 48,
                    h: 48,
                },
                Rect {
                    x: 0,
                    y: 192,
                    w: 48,
                    h: 48,
                },
                Rect {
                    x: 0,
                    y: 240,
                    w: 48,
                    h: 48,
                }],
                current_index: 0,
                start_time: 0,
                repeat: true,
            }]),
            Vec2i(10, 50),
            vec![Rect{
                x: 4,
                y: 30,
                w: 25,
                h: 15,
            }, Rect{
                x: 14,
                y: 14,
                w: 14,
                h: 31, 
            }],
            false,
            false,
            false,
        ),
        Sprite::new( // Laser 1
            &laser,
            Animation::new(vec![ AnimationState{
                frames: vec![   
                    Rect {
                    x: 0,
                    y: 0,
                    w: 20,
                    h: 10,
                }],
                current_index: 0,
                start_time: 0,
                repeat: false,
            }]),
            Vec2i(300, -20),
            vec![Rect {
                x: 0,
                y: 0,
                w: 20,
                h: 10,
            }],
            false,
            false,
            false,
        ),
        Sprite::new( // Laser 2
            &laser,
            Animation::new(vec![ AnimationState{
                frames: vec![   
                    Rect {
                    x: 0,
                    y: 0,
                    w: 20,
                    h: 10,
                }],
                current_index: 0,
                start_time: 0,
                repeat: false,
            }]),
            Vec2i(300, -20),
            vec![Rect {
                x: 0,
                y: 0,
                w: 20,
                h: 10,
            }],
            false,
            false,
            false,
        ),
        Sprite::new( // Laser 3
            &laser,
            Animation::new(vec![ AnimationState{
                frames: vec![   
                    Rect {
                    x: 0,
                    y: 0,
                    w: 20,
                    h: 10,
                }],
                current_index: 0,
                start_time: 0,
                repeat: false,
            }]),
            Vec2i(300, -20),
            vec![Rect {
                x: 0,
                y: 0,
                w: 20,
                h: 10,
            }],
            false,
            false,
            false,
        ),
        Sprite::new(
            &battery,
            Animation::new(vec![AnimationState {
                frames: vec![Rect {
                    x: 0,
                    y: 0,
                    w: 15,
                    h: 20,
                }],
                current_index: 0,
                start_time: 0,
                repeat: false,
            } , 
            AnimationState {
                frames: vec![Rect {
                    x: 15,
                     y: 0,
                     w: 15,
                     h: 20,
                }],
                current_index: 0,
                start_time: 0,
                repeat: false,
            },
            AnimationState {
                frames: vec![Rect {
                    x: 30,
                    y: 0,
                    w: 15,
                    h: 20,
                }],
                current_index: 0,
                start_time: 0,
                repeat: false,
            },
            AnimationState {
                frames: vec![Rect {
                    x: 45,
                    y: 0,
                    w: 15,
                    h: 20,
                }],
                current_index: 0,
                start_time: 0,
                repeat: false,
            }]),
            Vec2i(220, 10),
            vec![],
            false,
            false,
            false,
        ),
        Sprite::new(
            &asteroid,
            Animation::new(vec![
                AnimationState{
                    frames: vec![Rect {
                        x: 0,
                        y: 0,
                        w: 32,
                        h: 32,
                    }],
                    current_index: 0,
                    start_time: 0,
                    repeat: false,
                }]),
            Vec2i(400, 50),
            vec![Rect{
                x:2,
                y:4,
                w:28,
                h:19,
            }, Rect{
                x:7,
                y:2,
                w:19,
                h:25,
            }],
            false,
            false,
            true,
        ),
        Sprite::new(
            &ship,
            Animation::new(vec![ AnimationState{
                frames: vec![Rect {
                    x: 0,
                    y: 0,
                    w: 200,
                    h: 200,
                }],
                current_index: 0,
                start_time: 0,
                repeat: false,
            }]),
            Vec2i(600, 50),
            vec![],
            false,
            false,
            false,
        ),
        Sprite::new(
            &break_asteroid,
            Animation::new(vec![ AnimationState{
                frames: vec![
                    Rect {
                        x: 0,
                        y: 0,
                        w: 32,
                        h: 32,
                    }
                ],
                current_index: 0,
                start_time: 0,
                repeat: false,
            }]),
            Vec2i(250, 50),
            vec![Rect{
                x: 1,
                y: 11,
                w:29,
                h: 12
            }, Rect{
                x: 10,
                y: 8,
                w: 20,
                h: 17
            }],
            false,
            true,
            true,
        )],
        textures: vec![menu_1,menu_2, game_over],
        scroll: Vec2i(0,0),
        level: 0,
        current_tex: 0,
        shots_left: 3,
        shot_cool_down: 0,
        shot_index:0,
        frame: 0
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
    let maps = vec![
        Tilemap::new(
            Vec2i(0, 0),
            (64, 6),
            &tileset,
            make_map(),
        )  
    ];
    // How many frames have we simulated?
    let mut frame_count: usize = 0;
    // How many unsimulated frames have we saved up?
    let mut available_time = 0.0;
    // Track beginning of play
    let start = Instant::now();
    // Track end of the last frame
    let mut since = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let mut screen = Screen::wrap(pixels.get_frame(), WIDTH, HEIGHT, DEPTH, Vec2i(0,0));
            screen.clear(Rgba(255, 255, 255, 255));
            screen.position = state.scroll;

            if state.level == 0 {
                update_menu(&mut state, &input);
                screen.bitblt(&state.textures[state.current_tex],  // TODO: JUST MAKE A FUNCTION TO PROVIDE THE TEXTURE
                    Rect{ x: 0,
                    y: 0,
                    w: 240,
                    h: 240}, Vec2i(0, 0));

            } else if state.level == 5 {
                state.scroll = Vec2i(0,0);
                screen.bitblt(&state.textures[2],  // TODO: JUST MAKE A FUNCTION TO PROVIDE THE TEXTURE
                    Rect{ x: 0,
                    y: 0,
                    w: 240,
                    h: 240}, Vec2i(0, 0));
            } else {
                for map in maps.iter(){
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

            update_game(&mut state, &input, frame_count);
            // Increment the frame counter
            frame_count += 1;
            state.frame = frame_count;
        }
        // Request redraw
        window.request_redraw();
        // When did the last frame end?
        since = Instant::now();
    });
}

fn draw_game(state: &mut GameState, screen: &mut Screen, frame_number: usize) {

    for s in state.sprites.iter_mut() {
        s.animate(frame_number);
        screen.draw_sprite(s);
    }

}

fn update_game(state: &mut GameState, input: &WinitInputHelper, frame: usize) {
    // Player control goes here

    let bottom_border = 165;

    if input.key_held(VirtualKeyCode::Up) && !(state.sprites[0].position.1 > bottom_border) 
    && !(state.sprites[0].position.1 < 0)  {
        if state.sprites[0].vy > 0.0{
            state.sprites[0].vy /=3.0;
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
            state.sprites[0].vy +=0.1;
        }
    }

    if input.key_pressed(VirtualKeyCode::Space) && state.shot_cool_down == 0 {
        handle_shot(state);
    }

    // Scroll & Movement
    if state.level == 1 || state.level == 2 || state.level == 3 {
        // Scroll camera
        state.scroll = Vec2i(state.scroll.0 +2 , state.scroll.1);

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
        for i in 1..4{
            state.sprites[i].position.0 += 4;
            for hit_box in &mut state.sprites[i].hit_boxes{
                hit_box.x += 4;
            }
            // Move laser out of screen so it doesn't collide with obstacles that haven't loaded yet
            // Once out of the screen update our shot
            if state.sprites[i].position.0 == (state.scroll.0 + 240) {
                state.sprites[i].position.1 = -20;
                if state.shots_left < 3 {
                    state.shots_left += 1;
                    state.sprites[4].animation.index -= 1;   
                }
            }
        }
        
        // Move battery
        state.sprites[4].position.0 += 2;
        state.shot_cool_down  = (state.shot_cool_down - 1).max(0);
    }
    
    laser_contacts(&mut state.sprites);
    let colliding_sprite = player_contacts(&state.sprites);
    if colliding_sprite > -1 {
        thread::sleep(time::Duration::from_millis(500));
        state.level = 5;
    }

    // let length = state.sprites.len();
    // let mut counter: i32 = (length - 1) as i32;
    // while counter >= 0 {
    //     if state.sprites[counter as usize].exploded_counter > 10{
    //         state.sprites.remove(counter as usize);   
    //     } 
    //     counter = counter - 1;
    // }


 

}






// Function that takes care of moving around in the menu
fn update_menu(state: &mut GameState, input: &WinitInputHelper){
    if input.key_pressed(VirtualKeyCode::Up) {
        state.current_tex += 1;
        state.current_tex %= 2;
    }
    if input.key_pressed(VirtualKeyCode::Down) {
        state.current_tex += 1;
        state.current_tex %= 2;
    }
    if input.key_pressed(VirtualKeyCode::Return) {
        state.level = 1;
        for s in state.sprites.iter_mut() {
            s.animation.set_state(0, state.frame )
        } 
    }
}

fn make_map()->Vec<usize>{
    // 64 * 8 dims 
    // 7 rows are sky, 1 row is floor
    // 3 & 2 make the "sky" 
    // 0 & 1 make the floor
    let mut rng = rand::thread_rng();
    let mut sky: Vec<usize> =   (0..256).map(|_| (rng.gen_range(0, 4) as usize)).collect();
    let mut floor:  Vec<usize> =   (0..128).map(|_| (rng.gen_range(4, 8) as usize)).collect();
    sky.append(&mut floor);
    sky
}

fn handle_shot(state: &mut GameState) {
    // If we have a shot, shoot 
    if state.shots_left > 0 {
        state.sprites[4].animation.index += 1 ; // Update battery
        let i = (state.shot_index % 3) +1  ;
        state.sprites[i].position = state.sprites[0].position;
        state.sprites[i].position.0 += 30;
        state.sprites[i].position.1 += 20;
        let x_pos = state.sprites[i].position.0;
        let y_pos = state.sprites[i].position.1;
        for hit_box in &mut state.sprites[i].hit_boxes{
            hit_box.x = x_pos;
            hit_box.y = y_pos;
        }
        state.shots_left -= 1;
        state.shot_cool_down = 20;
        state.shot_index = state.shot_index +1;
    }
}