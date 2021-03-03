use crate::types::Rect;
use crate::sprite::Sprite;

pub fn player_contacts(sprites: &Vec<Sprite>) -> i32 {
    // collide mobiles against mobiles
   let length = sprites.len();
    for x in 1..length {
        if rect_displacement(sprites[0].hit_box.clone(), sprites[x].hit_box.clone()){
            return x as i32;
        } 
    
    }
    return -1 as i32;
}


pub fn rect_displacement(r1: Rect, r2: Rect) -> bool {
    // Draw this out on paper to double check, but these quantities
    // will both be positive exactly when the conditions in rect_touching are true.
    let x_overlap = (r1.x+r1.w as i32).min(r2.x+r2.w as i32) - r1.x.max(r2.x);
    let y_overlap = (r1.y+r1.h as i32).min(r2.y+r2.h as i32) - r1.y.max(r2.y);
    if x_overlap >= 0 && y_overlap >= 0 {
        // This will return the magnitude of overlap in each axis.
        true
    } else {
        false
    }
}