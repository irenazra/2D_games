use crate::texture::Texture;
use crate::types::{Rect, Vec2i};
use crate::animation::Animation;
use std::rc::Rc;

pub struct Sprite {
    image: Rc<Texture>,
    pub animation:Animation,
    pub position: Vec2i,
    pub vy: f32,
    pub hit_box: Rect,
    pub frame_pos: usize
}

impl Sprite {
    pub fn new(image: &Rc<Texture>, animation: Animation, position: Vec2i) -> Self {
        Self {
            image: Rc::clone(image),
            animation,
            position,
            vy: 0.0,
            hit_box: Rect {
                x: position.0,
                y: position.1,
                w: image.width as u16,
                h: image.height as u16
            }, 
            frame_pos: 0
        }
    }
}

pub trait DrawSpriteExt {
    fn draw_sprite(&mut self, s: &Sprite);
}

use crate::screen::Screen;
impl<'fb> DrawSpriteExt for Screen<'fb> {
    fn draw_sprite(&mut self, s: &Sprite) {
        // This works because we're only using a public method of Screen here,
        // and the private fields of sprite are visible inside this module
        self.bitblt(&s.image, s.animation.frames[(s.frame_pos)], s.position);
    }
}