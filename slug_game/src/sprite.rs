use crate::texture::Texture;
use crate::types::{Rect, Vec2i};
use crate::animation::Animation;
use std::rc::Rc;

pub struct Sprite {
    image: Rc<Texture>,
    pub animation:Animation,
    pub position: Vec2i,
    pub frame_pos: usize
}

impl Sprite {
    pub fn new(image: &Rc<Texture>, animation: Animation, position: Vec2i) -> Self {
        Self {
            image: Rc::clone(image),
            animation,
            position,
            frame_pos: 0
        }
    }

    //This rotates between sprites and creates a continous motion, independent of the where the
    //character is moving
    pub fn update_frame_pos(&mut self)  {
        let length = self.animation.frames.len();
        let mut next_sprite:usize = self.frame_pos + 1;

        if next_sprite == length{
            next_sprite = 0;
        }

        self.frame_pos= next_sprite;
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