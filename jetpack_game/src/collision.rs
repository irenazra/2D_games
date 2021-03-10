use crate::types::Rect;
use crate::sprite::Sprite;

pub fn player_contacts(sprites: &Vec<Sprite>) -> i32 {
    // collide player against objects
    for i in 4..sprites.len() {
        for hit_box in &sprites[i].hit_boxes{
            if rect_displacement(sprites[0].hit_boxes[0].clone(), hit_box.clone()) && sprites[i].is_obstacle {
                return x as i32;
            } else if rect_displacement(sprites[0].hit_boxes[1].clone(), hit_box.clone()) && sprites[i].is_obstacle {
                return x as i32;
            } 
        }
    }
    return -1 as i32;
}

pub fn laser_contacts(sprites: &Vec<Sprite>){
    for laser in 1..4{
        for i in 4..sprites.len(){
            for hit_box in &sprites[i].hit_boxes{
                if rect_displacement(sprites[laser].hit_boxes[0].clone(), hit_box.clone()) && sprites[i].is_obstacle {
                    return x as i32;
                }
            }
        }
    }
}



pub fn rect_displacement(r1: Rect, r2: Rect) -> bool {
    // Draw this out on paper to double check, but these quantities
    // will both be positive exactly when the conditions in rect_touching are true.
    let x_overlap = (r1.x+r1.w as i32).min(r2.x+r2.w as i32) - r1.x.max(r2.x);
    let y_overlap = (r1.y+r1.h as i32).min(r2.y+r2.h as i32) - r1.y.max(r2.y);
    if x_overlap >= 0 && y_overlap >= 0 {
        true
    } else {
        false
    }
}