use crate::animation::Animation;
use crate::sprite::Sprite;
use crate::texture::Texture;
use crate::types::*;
use rand::Rng;
use std::path::Path;
use std::rc::Rc;

pub fn make_player() -> Sprite {
    let slug = Rc::new(Texture::with_file(Path::new("src/pixel_art/slug.png")));
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
    )
}

pub fn make_enemy() -> Sprite {
    let enemy = Rc::new(Texture::with_file(Path::new("src/pixel_art/enemy.png")));
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
        Vec2i(100, 100),
    )
}

pub fn make_core() -> Vec<Sprite> {
    let mut core: Vec<Sprite> = vec![];
    core.push(make_player());
    core.push(make_enemy());
    return core;
}
