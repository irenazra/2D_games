use crate::types::AnimationState;

pub struct Animation {
    pub states: Vec<AnimationState>,
    pub index: usize,

    // Do this for the exercise today!
    // You'll want to know the frames involved and the timing for each frame
    // But then there's also dynamic data, which might live in this struct or might live somewhere else
    // An Animation/AnimationState split could be fine, if AnimationState holds the start time and the present frame (or just the start time) and possibly a reference to the Animation
    // but there are lots of designs that will work!
}

impl Animation {
    pub fn new(states: Vec<AnimationState>) -> Self {
        Self {
            states,
            index: 0 
        }
    }
}

pub trait StartAnim{
    fn set_state(&mut self, state:usize, current_frame: usize,); 
}

impl<'fb> StartAnim for Animation{
    fn set_state(&mut self, state:usize, current_frame: usize,){
        self.index = state;
        self.states[state].start_time = current_frame;
    } 
}





pub trait AnimateExt {
    fn animate(&mut self, current_frame: usize);
}

use crate::sprite::Sprite;
impl<'fb> AnimateExt for Sprite{
    fn animate(&mut self, current_frame: usize){
        if self.animation.states[self.animation.index].repeat {
            if current_frame >= (self.animation.states[self.animation.index].start_time + 6) {
                self.animation.states[self.animation.index].start_time = current_frame;
                self.animation.states[self.animation.index].current_index += 1 ;
                self.animation.states[self.animation.index].current_index %= self.animation.states[self.animation.index].frames.len();
            }
        } else {
            if current_frame >= (self.animation.states[self.animation.index].start_time + 6) {
                self.animation.states[self.animation.index].start_time = current_frame;
                self.animation.states[self.animation.index].current_index += 1 ;
                if self.animation.states[self.animation.index].current_index  == self.animation.states[self.animation.index].frames.len() {
                    self.animation.states[self.animation.index].current_index -= 1;
                }
            }
        }
    }
}
