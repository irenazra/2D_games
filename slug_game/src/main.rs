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
use std::collections::HashSet;

// Whoa what's this?
// Mod without brackets looks for a nearby file.
mod screen;
// Then we can use as usual.  The screen module will have drawing utilities.
use screen::Screen;
// Collision will have our collision bodies and contact types
//mod collision;
// Lazy glob imports
//use collision::*;
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

mod tile;
use tile::*;

use crate::types::{Rect, Vec2i};


// Now this main module is just for the run-loop and rules processing.
struct GameState {

    
    // What data do we need for this game?  Wall positions?
    sprites: Vec<Sprite>,
    tilemap:Tilemap,
    covered_tiles: usize,
}
// seconds per frame
const DT: f64 = 1.0 / 60.0;

const WIDTH: usize = 240;
const HEIGHT: usize = 240;
const DEPTH: usize = 4;

fn main() {
    println!("{}","Entering main");
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


    let mut tiles = Rc::new(Texture::with_file(Path::new("src/pixel_art/slug_tiles.png")));

    //CREATING THE SLUG SPRITE
    let mut slug_sprites = Rc::new(Texture::with_file(Path::new("src/pixel_art/slug.png")));
 
    let mut slug_animation = Animation::new(vec![Rect {
        x: 0,
        y: 0,
        w: 48,
        h: 48,
    },   
      Rect {
        x: 48,
        y: 0,
        w: 48,
        h: 48,
    },           
      Rect {
        x: 0,
        y: 48,
        w: 48,
        h: 48,
    },
    Rect {
        x: 48,
        y: 48,
        w: 48,
        h: 48,
    }]);

    let mut slug_position = Vec2i(10, 50);


    let mut slug_sprite = Sprite::new(
        &slug_sprites,
        slug_animation,
        slug_position,
    );
    
    //CREATING THE ENEMY SPRITE
    let mut enemy_sprites = Rc::new(Texture::with_file(Path::new("src/pixel_art/enemy.png")));
 
    let mut enemy_animation = Animation::new(vec![Rect {
        x: 0,
        y: 0,
        w: 48,
        h: 48,
    },   
      Rect {
        x: 48,
        y: 0,
        w: 48,
        h: 48,
    },           
      Rect {
        x: 0,
        y: 48,
        w: 48,
        h: 48,
    },
    Rect {
        x: 48,
        y: 48,
        w: 48,
        h: 48,
    }]);

    let mut enemy_position = Vec2i(100, 100);


    let mut enemy_sprite = Sprite::new(
        &enemy_sprites,
        enemy_animation,
        enemy_position,
    );


  


    //Create the tiles
    let first_tile = tile::Tile { solid: true };
    let second_tile = tile::Tile { solid: true };
    let third_tile = tile::Tile { solid: true };
    let fourth_tile = tile::Tile { solid: true };
  

    let tile_set = Rc::new(tile::Tileset{tiles : vec![first_tile, second_tile, third_tile,fourth_tile], texture:tiles});

    let mut first_ID = tile::TileID(0);
    let mut second_ID = tile::TileID(1);
    let mut third_ID = tile::TileID(2);
    let mut fourth_ID = tile::TileID(3);

    //let mut map = Vec::new();

    // for w in 0..WIDTH*4 {
    //     for h in 0..HEIGHT*4 {
            //let mut rng = thread_rng();
            //let random = rng.gen_range(0,3);
            // if random == 0 {
            //     map.push(0);
            // } else if random == 1 {
            //     map.push(1); 
            // } else {
            //     map.push(3); 
            // }
    //     }
    // }

    

    let mut map = vec![1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,3,1,1,1,1,1,1,1,1,1];

    



    let mut tile_map = tile::Tilemap::new(
        Vec2i(0,0),
        ((5) ,(5)),
        &tile_set,
        map,
    );




    let mut state = GameState {
        // initial game state...
        sprites: vec![slug_sprite, enemy_sprite],
        tilemap: tile_map,
        covered_tiles: 0,
    };

    // How many frames have we simulated?
    //60 FRAMES PER SECOND
    let mut frame_count: usize = 0;
    // How many unsimulated frames have we saved up?
    let mut available_time = 0.0;
    // Track beginning of play
    let start = Instant::now();
    // Track end of the last frame
    let mut since = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        let tile_map_position = Vec2i(1 as i32,1 as i32);
        if let Event::RedrawRequested(_) = event {
            let mut screen = Screen::wrap(pixels.get_frame(), WIDTH, HEIGHT, DEPTH, tile_map_position);
            screen.clear(Rgba(0, 0, 0, 0));

            

            draw_game(&mut state, &mut screen,frame_count);

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

fn draw_game(state: &mut GameState, screen: &mut Screen,frame_number:usize) {
    // Call screen's drawing methods to render the game state
    screen.clear(Rgba(80, 80, 80, 255));

    state.tilemap.draw(screen);

    for s in state.sprites.iter_mut() {
        if frame_number%7 == 0 {
            s.update_frame_pos();
        }
        screen.draw_sprite(s);
    }

}

fn update_game(state: &mut GameState, input: &WinitInputHelper, frame: usize) {

    //UPDATES RELATED TO THE PLAYER
    let x1 = state.sprites[0].position.0;
    let x2 = state.sprites[0].position.0 + 48;
    let y1 = state.sprites[0].position.1;
    let y2 = state.sprites[0].position.1 + 48;

    if x1 < 2 {
        state.sprites[0].position.0 += 2;
    } 
    
    if x2 > 238 {
        state.sprites[0].position.0 -= 2;
    } 

    if y1 < 2 {
        state.sprites[0].position.1 += 2;
    }

    if y2 > 238 {
        state.sprites[0].position.1 -= 2;
    } 
    
    let top_left = Vec2i(x1 + 12,y1 + 8);
    let top_right = Vec2i(x2 -8,y1 + 8);
    let bottom_left = Vec2i(x1 + 12,y2 - 12);
    let bottom_right = Vec2i(x2 - 8,y2 - 12);

    let colliding_tile_tl = state.tilemap.tile_id_at(top_left);
    let colliding_tile_tr = state.tilemap.tile_id_at(top_right);
    let colliding_tile_bl = state.tilemap.tile_id_at(bottom_left);
    let colliding_tile_br = state.tilemap.tile_id_at(bottom_right);

    let mut collided_tiles = HashSet::new();
    collided_tiles.insert(colliding_tile_tl.0);
    collided_tiles.insert(colliding_tile_tr.0);
    collided_tiles.insert(colliding_tile_bl.0);
    collided_tiles.insert(colliding_tile_br.0);

    if collided_tiles.contains(&3) {
        println!("{}", "You died! Got too close to fire!");
    }

    //top left collides with a wall
    if colliding_tile_tl.0 == 0 {
        //bottom left collides with a wall
        if (colliding_tile_bl.0 == 0) {
            state.sprites[0].position.0 += 2;
        } else {
            state.sprites[0].position.1 += 2;
        }
    } 

    if colliding_tile_bl.0 == 0 {
        if colliding_tile_br.0 == 0 {
            state.sprites[0].position.1 += - 2;
        }  else {
            state.sprites[0].position.0 += 2;
        }
    }


    //top right collides with a wall
    if colliding_tile_tr.0 == 0 {
        //bottom right collides with a wall
        if (colliding_tile_br.0 == 0) {
            state.sprites[0].position.0 -= 2;
        } else {
            state.sprites[0].position.1 += 2;
        }
    }

    if colliding_tile_tl.0 == 1  {
        let index = state.tilemap.tile_index_at(top_left);
        state.covered_tiles += 1;
        state.tilemap.map[index] = TileID(2);
    }

    if colliding_tile_tr.0 == 1  {
        let index = state.tilemap.tile_index_at(top_right);
        state.covered_tiles += 1;
        state.tilemap.map[index] = TileID(2);
    }

    if colliding_tile_bl.0 == 1  {
        let index = state.tilemap.tile_index_at(bottom_left);
        state.covered_tiles += 1;
        state.tilemap.map[index] = TileID(2);
    }

    if colliding_tile_br.0 == 1  {
        let index = state.tilemap.tile_index_at(bottom_right);
        state.covered_tiles += 1;
        state.tilemap.map[index] = TileID(2);
    }




    // Player control goes here
    if input.key_held(VirtualKeyCode::Right) {
        state.sprites[0].position.0 += 2;
        state.sprites[0].frame_pos = 2;
    }
    if input.key_held(VirtualKeyCode::Left) {
        state.sprites[0].position.0 -= 2;
        state.sprites[0].frame_pos = 0;
    }
    if input.key_held(VirtualKeyCode::Up) {
        state.sprites[0].position.1 -= 2;

    }
    if input.key_held(VirtualKeyCode::Down) {
        state.sprites[0].position.1 += 2;

    }

    //ENEMY CONTROL

    let mut enemy_pos_x = state.sprites[1].position.0;
    let mut enemy_pos_y = state.sprites[1].position.1;

    let x_distance = x1 - enemy_pos_x;
    let y_distance = y1 - enemy_pos_y;

  

    //Create intelligent behaviour for the enemy
   
    //let mut rng = thread_rng();
    //let random = rng.gen_range(0,2);
    //if random == 0 {

    if x_distance > 0 {
            enemy_pos_x += 1
        } else if x_distance < 0 {
            enemy_pos_x -= 1
        
    } else {
        if y_distance > 0 {
            enemy_pos_y += 1
        } else if y_distance < 0 {
            enemy_pos_y -= 1
        }
    }

    if enemy_pos_x < 1 {
        enemy_pos_x += 1;
    } 
    
    if enemy_pos_x + 48 > 239 {
        enemy_pos_x -= 1;
    } 

    if enemy_pos_y < 1 {
        enemy_pos_y += 1;
    }

    if enemy_pos_y + 48 > 239 {
        enemy_pos_y -= 1;
    } 


    let tl = Vec2i(enemy_pos_x + 12,enemy_pos_y + 8);
    let tr = Vec2i(enemy_pos_x + 48 - 8,enemy_pos_y + 8);
    let bl = Vec2i(enemy_pos_x  + 12,enemy_pos_y + 48 - 12);
    let br = Vec2i(enemy_pos_x + 48 - 8,enemy_pos_y + 48 - 12);

    let c_tile_tl = state.tilemap.tile_id_at(tl);
    let c_tile_tr = state.tilemap.tile_id_at(tr);
    let c_tile_bl = state.tilemap.tile_id_at(bl);
    let c_tile_br = state.tilemap.tile_id_at(br);

      //top left collides with a wall
    if c_tile_tl.0 == 0 {
        //bottom left collides with a wall
        if (c_tile_bl.0 == 0) {
            enemy_pos_x += 1;
        } else {
            enemy_pos_y += 1;
        }
    } 

    if c_tile_bl.0 == 0 {
        if c_tile_br.0 == 0 {
            enemy_pos_y += - 1;
        }  else {
            enemy_pos_x += 1;
        }
    }

    //top right collides with a wall
    if c_tile_tr.0 == 0 {
        //bottom right collides with a wall
        if (c_tile_br.0 == 0) {
            enemy_pos_x -= 1;
        } else {
            enemy_pos_y += 1;
        }
    }

    state.sprites[1].position.0 = enemy_pos_x;
    state.sprites[1].position.1 = enemy_pos_y;





}



