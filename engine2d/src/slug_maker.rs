use crate::animation::Animation;
use crate::sprite::Sprite;
use crate::texture::Texture;
use crate::types::*;
use rand::Rng;
use std::path::Path;
use std::rc::Rc;
use crate::tile::*;

pub fn make_player() -> Sprite {
    let slug = Rc::new(Texture::with_file(Path::new("slug/slug.png")));
    Sprite::new(
        &slug,
        Animation::new(vec![
            AnimationState {
                frames: vec![
                    Rect {
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
                    },
                ],
                current_index: 0,
                start_time: 0,
                repeat: true,
            },
            AnimationState {
                frames: vec![
                    Rect {
                        x: 0, // Left facing slug
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
                ],
                current_index: 0,
                start_time: 0,
                repeat: true,
            },
            AnimationState {
                frames: vec![
                    Rect {
                        // Right facing slug
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
                    },
                ],
                current_index: 0,
                start_time: 0,
                repeat: true,
            },
        ]),
        Vec2i(10, 50),
        vec![Rect {
            x: 12,
            y: 16,
            w: 12,
            h: 16,
        }],
        false,
        false,
        false,
    )
}

pub fn make_enemy() -> Sprite {
    let enemy = Rc::new(Texture::with_file(Path::new("slug/enemy.png")));
    Sprite::new(
        &enemy,
        Animation::new(vec![
            AnimationState {
                frames: vec![
                    Rect {
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
                    },
                ],
                current_index: 0,
                start_time: 0,
                repeat: true,
            },
            AnimationState {
                frames: vec![
                    Rect {
                        x: 0, // Left facing slug
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
                ],
                current_index: 0,
                start_time: 0,
                repeat: true,
            },
            AnimationState {
                frames: vec![
                    Rect {
                        // Right facing slug
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
                    },
                ],
                current_index: 0,
                start_time: 0,
                repeat: true,
            },
        ]),
        Vec2i(100, 160),
        vec![Rect {
            x: 12,
            y: 16,
            w: 12,
            h: 16,
        }],
        false,
        false,
        true,
    )
}

pub fn make_core() -> Vec<Sprite> {
    let mut core: Vec<Sprite> = vec![];
    core.push(make_player());
    core.push(make_enemy());
    return core;
}

pub fn make_menus() ->  Vec<Rc<Texture>> {
    let menu_1 = Rc::new(Texture::with_file(Path::new("slug/screens/play.png")));
    let menu_2 = Rc::new(Texture::with_file(Path::new("slug/screens/load.png")));
    let menu_3 = Rc::new(Texture::with_file(Path::new("slug/screens/tut.png")));
    let help = Rc::new(Texture::with_file(Path::new("slug/screens/help.png")));
    let game_over = Rc::new(Texture::with_file(Path::new(
        "slug/screens/game_over.png",
    )));
    let win = Rc::new(Texture::with_file(Path::new("slug/screens/win.png")));
    return vec![menu_1, menu_2, menu_3, help, win, game_over];
}

pub fn reset_tiles() -> Tilemap {
    let mut tiles = Rc::new(Texture::with_file(Path::new("slug/slug_tiles.png")));

    //Create the tiles
    let first_tile = Tile { solid: true };
    let second_tile = Tile { solid: true };
    let third_tile = Tile { solid: true };
    let fourth_tile = Tile { solid: true };
  
    let mut map = vec![1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,3,1,1,1,1,1,1,1,1,1, 1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,3,1,1,1,1,1,1,1,1,1, 1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,3,1,1,1,1,1,1,1,1,1, 1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,3,1,1,1,1,1,1,1,1,1,];
    let tile_set = Rc::new(Tileset{tiles : vec![first_tile, second_tile, third_tile,fourth_tile], texture:tiles});
    let tilemap= Tilemap::new(
        Vec2i(0,0),
        ((10) ,(10)),
        &tile_set,
        map,
    );
    return tilemap
}