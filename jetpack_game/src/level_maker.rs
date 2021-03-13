use crate::animation::Animation;
use crate::sprite::Sprite;
use crate::texture::Texture;
use crate::types::*;
use rand::Rng;
use std::path::Path;
use std::rc::Rc;

pub fn make_asteroid(position: Vec2i) -> Sprite {
    let asteroid = Rc::new(Texture::with_file(Path::new("content/asteroid.png")));
    Sprite::new(
        &asteroid,
        Animation::new(vec![AnimationState {
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
        position,
        vec![
            Rect {
                x: 2,
                y: 4,
                w: 28,
                h: 19,
            },
            Rect {
                x: 7,
                y: 2,
                w: 19,
                h: 25,
            },
        ],
        false,
        false,
        true,
    )
}

pub fn make_break_asteroid(position: Vec2i) -> Sprite {
    let break_asteroid = Rc::new(Texture::with_file(Path::new("content/b_asteroid.png")));
    Sprite::new(
        &break_asteroid,
        Animation::new(vec![
            AnimationState {
                frames: vec![Rect {
                    x: 0,
                    y: 0,
                    w: 32,
                    h: 32,
                }],
                current_index: 0,
                start_time: 0,
                repeat: false,
            },
            AnimationState {
                frames: vec![Rect {
                    x: 32,
                    y: 0,
                    w: 32,
                    h: 32,
                }],
                current_index: 0,
                start_time: 0,
                repeat: false,
            },
        ]),
        position,
        vec![
            Rect {
                x: 1,
                y: 11,
                w: 29,
                h: 12,
            },
            Rect {
                x: 10,
                y: 8,
                w: 20,
                h: 17,
            },
        ],
        false,
        true,
        true,
    )
}

pub fn make_little_asteroid(position: Vec2i) -> Sprite {
    let asteroid = Rc::new(Texture::with_file(Path::new("content/l_asteroid.png")));
    Sprite::new(
        &asteroid,
        Animation::new(vec![AnimationState {
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
        position,
        vec![Rect {
            x: 1,
            y: 1,
            w: 21,
            h: 17,
        }],
        false,
        false,
        true,
    )
}

pub fn make_laser() -> Sprite {
    let laser = Rc::new(Texture::with_file(Path::new("content/laser.png")));
    Sprite::new(
        // Laser 1
        &laser,
        Animation::new(vec![AnimationState {
            frames: vec![Rect {
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
    )
}

pub fn make_ship() -> Sprite {
    let ship = Rc::new(Texture::with_file(Path::new("content/ship.png")));
    Sprite::new(
        &ship,
        Animation::new(vec![AnimationState {
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
        Vec2i(2700, 55),
        vec![],
        false,
        false,
        false,
    )
}

pub fn make_player() -> Sprite {
    let llama = Rc::new(Texture::with_file(Path::new("content/llama.png")));
    Sprite::new(
        &llama,
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
                        x: 0,
                        y: 48,
                        w: 48,
                        h: 48,
                    },
                    Rect {
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
                    },
                ],
                current_index: 0,
                start_time: 0,
                repeat: true,
            },
            AnimationState {
                frames: vec![
                    Rect {
                        x: 0,
                        y: 288,
                        w: 48,
                        h: 48,
                    },
                    Rect {
                        x: 0,
                        y: 336,
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
        vec![
            Rect {
                x: 4,
                y: 30,
                w: 25,
                h: 15,
            },
            Rect {
                x: 14,
                y: 14,
                w: 14,
                h: 31,
            },
        ],
        false,
        false,
        false,
    )
}

pub fn make_battery() -> Sprite {
    let battery = Rc::new(Texture::with_file(Path::new("content/battery.png")));
    Sprite::new(
        &battery,
        Animation::new(vec![
            AnimationState {
                frames: vec![Rect {
                    x: 0,
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
            },
        ]),
        Vec2i(220, 10),
        vec![],
        false,
        false,
        false,
    )
}

pub fn make_core() -> Vec<Sprite> {
    let mut core: Vec<Sprite> = vec![];
    core.push(make_player());
    for _ in 0..3 {
        core.push(make_laser());
    }
    core.push(make_battery());
    core.push(make_ship());
    return core;
}

pub fn make_map() -> Vec<usize> {
    // 64 * 8 dims
    // 7 rows are sky, 1 row is floor
    let mut rng = rand::thread_rng();
    let mut sky: Vec<usize> = (0..256).map(|_| (rng.gen_range(0, 4) as usize)).collect();
    let mut floor: Vec<usize> = (0..128).map(|_| (rng.gen_range(4, 8) as usize)).collect();
    sky.append(&mut floor);
    sky
}

pub fn make_menus() -> Vec<Rc<Texture>> {
    let menu_1 = Rc::new(Texture::with_file(Path::new("content/screens/play.png")));
    let menu_2 = Rc::new(Texture::with_file(Path::new("content/screens/load.png")));
    let menu_3 = Rc::new(Texture::with_file(Path::new("content/screens/tut.png")));
    let help = Rc::new(Texture::with_file(Path::new("content/screens/help.png")));
    let game_over = Rc::new(Texture::with_file(Path::new(
        "content/screens/game_over.png",
    )));
    let next = Rc::new(Texture::with_file(Path::new("content/screens/next.png")));
    let win = Rc::new(Texture::with_file(Path::new("content/screens/win.png")));
    return vec![menu_1, menu_2, menu_3, help, next, win, game_over];
}

// BOTTOM = 165
// Asteroids start at 200
pub fn level_1() -> Vec<Sprite> {
    let mut sprites1 = make_core();
    let asteroids = vec![Vec2i(150, 50), Vec2i(190, 10), Vec2i(300, 180)];
    for position in asteroids {
        sprites1.push(make_asteroid(position));
    }

    return sprites1;
}

pub fn level_2() -> Vec<Sprite> {
    let mut sprites2 = make_core();
    let asteroids = vec![
        Vec2i(150, 50),
        Vec2i(190, 10),
        Vec2i(300, 180),
        Vec2i(400, 180),
    ];
    for position in asteroids {
        sprites2.push(make_asteroid(position));
    }
    return sprites2;
}

pub fn level_3() -> Vec<Sprite> {
    let mut sprites = make_core();
    let asteroids = vec![Vec2i(150, 50), Vec2i(190, 10), Vec2i(300, 180)];
    for position in asteroids {
        sprites.push(make_asteroid(position));
    }
    return sprites;
}
