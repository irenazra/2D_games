use pixels::{Pixels, SurfaceTexture};
use std::path::Path;
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
use animation::Animation;
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
}
// seconds per frame
const DT: f64 = 1.0 / 60.0;

const WIDTH: usize = 128;
const HEIGHT: usize = 128;
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
    let king = Rc::new(Texture::with_file(Path::new("content/king.png")));
    let asteroid = Rc::new(Texture::with_file(Path::new("content/asteroid.png")));  
    let menu_1 = Rc::new(Texture::with_file(Path::new("content/screens/play.png")));
    let menu_2 = Rc::new(Texture::with_file(Path::new("content/screens/load.png")));
    let game_over = Rc::new(Texture::with_file(Path::new("content/screens/game_over.png")));
    let mut state = GameState {
        // initial game state...
        sprites: vec![Sprite::new(
            &king,
            Animation::new(vec![Rect {
                x: 0,
                y: 16,
                w: 16,
                h: 16,
            },           
              Rect {
                x: 16,
                y: 16,
                w: 16,
                h: 16,
            }]),
            Vec2i(10, 50),
        ),
        Sprite::new(
            &asteroid,
            Animation::new(vec![Rect {
                x: 0,
                y: 0,
                w: 16,
                h: 16,
            }]),
            Vec2i(300, 50),
        )],
        textures: vec![menu_1,menu_2, game_over],
        scroll: Vec2i(0,0),
        level: 0,
        current_tex: 0
    };
    let tex = Rc::new(Texture::with_file(Path::new("content/tileset.png")));
    let tileset = Rc::new(Tileset::new(
        vec![
            Tile { solid: false },
            Tile { solid: true },
            Tile { solid: true },
            Tile { solid: true },
        ],
        &tex,
    ));
    let maps = vec![
        Tilemap::new(
            Vec2i(0, 0),
            (64, 8),
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
                    w: 128,
                    h: 128}, Vec2i(0, 0));
            } else if state.level == 5 {
                state.scroll = Vec2i(0,0);
                screen.bitblt(&state.textures[2],  // TODO: JUST MAKE A FUNCTION TO PROVIDE THE TEXTURE
                    Rect{ x: 0,
                    y: 0,
                    w: 128,
                    h: 128}, Vec2i(0, 0));
            } else {
                for map in maps.iter(){
                    map.draw(&mut screen);
                }
                draw_game(&state, &mut screen);
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
        }
        // Request redraw
        window.request_redraw();
        // When did the last frame end?
        since = Instant::now();
    });
}

fn draw_game(state: &GameState, screen: &mut Screen) {
        for s in state.sprites.iter() {
        screen.draw_sprite(s);
    }

}

fn update_game(state: &mut GameState, input: &WinitInputHelper, frame: usize) {
    // Player control goes here

    if input.key_held(VirtualKeyCode::Up) && !(state.sprites[0].position.1 > 96) 
    && !(state.sprites[0].position.1 < 0)  {
        if state.sprites[0].vy > 0.0{
            state.sprites[0].vy /=3.0;
        }
        state.sprites[0].vy -= 0.2;
    } else {
        if state.sprites[0].position.1 <= 0 {
            state.sprites[0].position.1 = 0;
        }
        if state.sprites[0].position.1 >= 96  {
            state.sprites[0].vy = 0.0;
            state.sprites[0].position.1 = 96;
        } else {
            state.sprites[0].vy +=0.1;
        }
    }
    if state.level == 1 || state.level == 2 || state.level == 3 {
        // Scroll camera
        state.scroll = Vec2i(state.scroll.0 +1 , state.scroll.1);

        // Move Sprite  
        state.sprites[0].position.0 += 1;
        state.sprites[0].position.1 += state.sprites[0].vy.min(2.0) as i32; // minimum set to have a "max speed"
        // Move Hitbox
        state.sprites[0].hit_box.x = state.sprites[0].position.0;
        state.sprites[0].hit_box.y = state.sprites[0].position.1;
    }
    if player_contacts(&state.sprites){
        state.level = 5
    }

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
    if input.key_pressed(VirtualKeyCode::Return){
        state.level = 1;
    }
}

fn make_map()->Vec<usize>{
    // 64 * 8 dims 
    // 7 rows are sky, 1 row is floor
    // 3 & 2 make the "sky" 
    // 0 & 1 make the floor
    let mut rng = rand::thread_rng();
    let mut sky: Vec<usize> =   (0..448).map(|_| (rng.gen_range(0, 2) as usize)).collect();
    let mut floor:  Vec<usize> =   (0..64).map(|_| (rng.gen_range(2, 4) as usize)).collect();
    sky.append(&mut floor);
    sky
}