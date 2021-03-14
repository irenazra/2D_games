use crate::animation::Animation;
use crate::sprite::Sprite;
use crate::texture::Texture;
use crate::types::*;
use rand::Rng;
use std::path::Path;
use std::rc::Rc;

pub fn make_asteroid(position: Vec2i) -> Sprite {
    let asteroid = Rc::new(Texture::with_file(Path::new("content/sprites/asteroid.png")));
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
                x: 5,
                y: 4,
                w: 28,
                h: 15,
            },
            Rect {
                x: 7,
                y: 2,
                w: 19,
                h: 20,
            },
        ],
        false,
        false,
        true,
    )
}

pub fn make_break_asteroid(position: Vec2i) -> Sprite {
    let break_asteroid = Rc::new(Texture::with_file(Path::new("content/sprites/b_asteroid.png")));
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
                x: 2,
                y: 11,
                w: 29,
                h: 10,
            },
            Rect {
                x: 10,
                y: 7,
                w: 20,
                h: 12,
            },
        ],
        false,
        true,
        true,
    )
}

pub fn make_little_asteroid(position: Vec2i) -> Sprite {
    let asteroid = Rc::new(Texture::with_file(Path::new("content/sprites/l_asteroid.png")));
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
            w: 17,
            h: 10,
        }],
        false,
        false,
        true,
    )
}

pub fn make_laser() -> Sprite {
    let laser = Rc::new(Texture::with_file(Path::new("content/sprites/laser.png")));
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
            w: 15,
            h: 10,
        }],
        false,
        false,
        false,
    )
}

pub fn make_ship() -> Sprite {
    let ship = Rc::new(Texture::with_file(Path::new("content/sprites/ship.png")));
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
        Vec2i(2500, 55),
        vec![],
        false,
        false,
        false,
    )
}

pub fn make_player() -> Sprite {
    let llama = Rc::new(Texture::with_file(Path::new("content/sprites/llama.png")));
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
                y: 31,
                w: 22,
                h: 15,
            },
            Rect {
                x: 14,
                y: 15,
                w: 12,
                h: 25,
            },
        ],
        false,
        false,
        false,
    )
}

pub fn make_battery() -> Sprite {
    let battery = Rc::new(Texture::with_file(Path::new("content/sprites/battery.png")));
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

pub fn level_1() -> Vec<Sprite> {
    let mut sprites = make_core();
    let asteroids = vec![
        Vec2i(150, 50),
        Vec2i(300, 160),
        Vec2i(300, 0),
        Vec2i(350, 40),
        Vec2i(600, 90),
        Vec2i(700, 165),
        Vec2i(750, 140),
        Vec2i(800, 10),
        Vec2i(900, 30),
        Vec2i(1000, 90),
        Vec2i(1400, 160),
        Vec2i(1404, 0),
        Vec2i(1500, 10),
        Vec2i(1700, 99),
        Vec2i(1820, 106),
        Vec2i(2150, 0),
        Vec2i(2150, 50),
        Vec2i(2150, 100),
        ];
        let break_asteroids = vec![
            Vec2i(525, 44),
            Vec2i(500, 100),
            Vec2i(190, 10),
            Vec2i(2150, 152),
            Vec2i(400, 75),
            Vec2i(1200, 150),
            Vec2i(1300, 66),
            Vec2i(1000, 0),
            Vec2i(2150, 170),
            ];

        let little_asteroids = vec![
            Vec2i(150, 0),
            Vec2i(190, 40),
            Vec2i(340, 160),
            Vec2i(720, 155),
            Vec2i(850, 10),
            Vec2i(1200, 0),
            Vec2i(1300, 0),
            Vec2i(1700, 120),
            Vec2i(1820, 0),
            ];
            
            for position in asteroids {
                sprites.push(make_asteroid(position));
            }
            for position in little_asteroids{
                sprites.push(make_little_asteroid(position));
    }
    for position in break_asteroids{
        sprites.push(make_break_asteroid(position));
    }
    
    return sprites;
}

pub fn level_2() -> Vec<Sprite> {
    let mut sprites = make_core();
    let asteroids = vec![
        Vec2i(170, 155),
        Vec2i(190, 125),
        Vec2i(300, 0),
        Vec2i(300, 30),
        Vec2i(350, 40),
        Vec2i(400, 75),
        Vec2i(600, 90),
        Vec2i(800, 10),
        Vec2i(1000, 0),
        Vec2i(1000, 40),
        Vec2i(1000, 90),
        Vec2i(1450, 150),
        Vec2i(1450, 0),
        Vec2i(1450, 45),
        Vec2i(1450, 150),
        Vec2i(1770, 165),
        Vec2i(1790, 130),
        Vec2i(1800, 100),
        Vec2i(1800, 70),
        Vec2i(2150, 0),
        Vec2i(2150, 50),
        Vec2i(2150, 100),
    ];
    let break_asteroids = vec![
        Vec2i(1000, 120),
        Vec2i(1010, 165),
        Vec2i(1450, 75),
        Vec2i(1450, 115),
        Vec2i(1800, 20),
        Vec2i(2150, 170),
    ];
    let little_asteroids = vec![
        Vec2i(150, 170),
        Vec2i(190, 20),
        Vec2i(250, 170),
        Vec2i(350, 10),
        Vec2i(390, 130),
        Vec2i(475, 150),
        Vec2i(600, 90),
        Vec2i(600, 20),
        Vec2i(850, 173),
        Vec2i(1480, 0),
        ];

    for position in asteroids {
        sprites.push(make_asteroid(position));
    }
    for position in little_asteroids{
        sprites.push(make_little_asteroid(position));
    }
    for position in break_asteroids{
        sprites.push(make_break_asteroid(position));
    }
    
    return sprites;
}

pub fn level_3() -> Vec<Sprite> {
    let mut sprites = make_core();
    let asteroids = vec![
        Vec2i(210, 30),
        Vec2i(225, 140),
        Vec2i(250, 0),
        Vec2i(261, 170),
        Vec2i(320, 172),
        Vec2i(350, 150),
        Vec2i(400, 100),
        Vec2i(420, 172),
        Vec2i(450, 0),
        Vec2i(500, 20),
        Vec2i(540, 19),
        Vec2i(660, 175),
        Vec2i(710, 130),
        Vec2i(720, 150),
        Vec2i(830, 150),
        Vec2i(800, 100),
        Vec2i(880, 30),
        Vec2i(940, 0),
        Vec2i(1045, 50),
        Vec2i(1110, 130),
        Vec2i(1190, 170),
        Vec2i(1210, 110),
        Vec2i(1250, 80),
        Vec2i(1330, 0),
        Vec2i(1370, 90),
        Vec2i(1400, 170),
        Vec2i(1450, 70),
        Vec2i(1500, 150),
        Vec2i(1600, 25),
        Vec2i(1610, 170),
        Vec2i(1660, 100),
        Vec2i(1780, 25),
        Vec2i(1850, 60),
        Vec2i(1853, 90),
        Vec2i(1860, 120),
        Vec2i(1970, 165),
        Vec2i(2050, 40),
        Vec2i(2100, 70),
        Vec2i(2100, 165),
        Vec2i(2200, 165),
        Vec2i(2200, 140),
        Vec2i(2395, 0),
        ];
        let break_asteroids = vec![
            Vec2i(240, 100),
            Vec2i(1470, 15),
            Vec2i(283, 140),
            Vec2i(493, 70),
            Vec2i(570, 50),
            Vec2i(565, 140),
            Vec2i(600, 20),
            Vec2i(670, 100),
            Vec2i(950, 145),
            Vec2i(960, 170),
            Vec2i(1010, 165),
            Vec2i(1690, 20),
            Vec2i(1690, 130),
            Vec2i(1870, 170),
            Vec2i(2230, 20),
            Vec2i(2395, 100),
            ];
            let little_asteroids = vec![
                Vec2i(2395, 160),
                Vec2i(2210, 115),
                Vec2i(1560, 5),
                Vec2i(1150, 40),
                Vec2i(2010, 165),
                Vec2i(2200, 85),
                Vec2i(1700, 140),
                Vec2i(1320, 130),
                Vec2i(1000, 10),
                Vec2i(910, 70),
                Vec2i(275, 30),
                Vec2i(620, 48),
                Vec2i(200, 170),
                Vec2i(1420, 120),
                Vec2i(200, 0),
                Vec2i(790, 22),
                Vec2i(200, 60),
                ];
                
    for position in asteroids {
        sprites.push(make_asteroid(position));
    }
    for position in little_asteroids{
        sprites.push(make_little_asteroid(position));
    }
    for position in break_asteroids{
        sprites.push(make_break_asteroid(position));
    }
    
    return sprites;
}
