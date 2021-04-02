use crate::animation::Animation;
use crate::texture::Texture;
use crate::types::{Rect, Vec2i};
use std::rc::Rc;

pub struct Sprite {
    image: Rc<Texture>,
    pub animation: Animation,
    pub position: Vec2i,
    pub frame_pos: usize,
    pub hit_boxes: Vec<Rect>,
    pub exploded: bool,
    pub is_explodable: bool,
    pub is_obstacle: bool,
}

impl Sprite {
    pub fn new(image: &Rc<Texture>, animation: Animation, position: Vec2i,hit_b: Vec<Rect>, exp: bool, exp_able:bool, obs:bool ) -> Self {
        Self {
            image: Rc::clone(image),
            animation,
            position,
            frame_pos: 0,
            hit_boxes: hit_b,
            exploded:exp,
            is_explodable: exp_able,
            is_obstacle: obs,
        }
    }

    //This rotates between sprites and creates a continous motion, independent of the where the
    //character is moving
    // pub fn update_frame_pos(&mut self)  {
    //     let length = self.animation.frames.len();
    //     let mut next_sprite:usize = self.frame_pos + 1;

    //     if next_sprite == length{
    //         next_sprite = 0;
    //     }

    //     self.frame_pos= next_sprite;
    // }
}

pub trait DrawSpriteExt {
    fn draw_sprite(&mut self, s: &Sprite);
}

use crate::screen::Screen;
impl<'fb> DrawSpriteExt for Screen<'fb> {
    fn draw_sprite(&mut self, s: &Sprite) {
        // This works because we're only using a public method of Screen here,
        // and the private fields of sprite are visible inside this module
        self.bitblt(
            &s.image,
            s.animation.states[s.animation.index].frames
                [s.animation.states[s.animation.index].current_index],
            s.position,
        );
    }
}
