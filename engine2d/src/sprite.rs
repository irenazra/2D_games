use crate::animation::Animation;
use crate::texture::Texture;
use crate::types::{Rect, Vec2i};
use std::rc::Rc;

pub struct Sprite {
    pub image: Rc<Texture>,
    pub animation: Animation,
    pub position: Vec2i,
    pub vy: f32,
    pub hit_boxes: Vec<Rect>,
    pub exploded: bool,
    pub is_explodable: bool,
    pub is_obstacle: bool,
}

impl Sprite {
    pub fn new(
        image: &Rc<Texture>,
        animation: Animation,
        Vec2i(x, y): Vec2i,
        mut hit_boxes: Vec<Rect>,
        exploded: bool,
        is_explodable: bool,
        is_obstacle: bool,
    ) -> Self {
        for mut rect in &mut hit_boxes {
            rect.x += x;
            rect.y += y;
        }
        Self {
            image: Rc::clone(image),
            animation,
            position: Vec2i(x, y),
            vy: 0.0,
            hit_boxes,
            exploded: exploded,
            is_explodable,
            is_obstacle,
        }
    }

    // //This rotates between sprites and creates a continous motion, independent of the where the
    // //character is moving
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
    fn draw_sprite(&mut self, s: &mut Sprite);
}

use crate::screen::Screen;
impl<'fb> DrawSpriteExt for Screen<'fb> {
    fn draw_sprite(&mut self, s: &mut Sprite) {
        // This works because we're only using a public method of Screen here,
        // and the private fields of sprite are visible inside this module
        // if s.exploded {
        //     s.exploded_counter = s.exploded_counter + 1;
        //     if s.image.width == 48 {
        //         s.image =  Rc::new(Texture::with_file(Path::new("content/big_explosion.png")));
        //     } else {
        //         s.image = Rc::new(Texture::with_file(Path::new("content/small_explosion.png")));
        //     }

        // }
        self.bitblt(
            &s.image,
            s.animation.states[s.animation.index].frames
                [s.animation.states[s.animation.index].current_index],
            s.position,
        );
    }
}
