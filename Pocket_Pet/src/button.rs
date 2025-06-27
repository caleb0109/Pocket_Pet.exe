use crate::BorshDeserialize;
use crate::BorshSerialize;
use turbo::prelude::*;


#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ActionButton {
    pub hitbox: (i32, i32, i32, i32),
    pub text: String,
    pub hovered: bool,
    pub action: bool,
    pub luxary: bool,
    pub count: u32,
}

impl ActionButton {
    pub fn new (text: &str, hitbox: (i32, i32, i32, i32), act: bool) -> Self {
        Self {
            hitbox, // x, y, w, h
            text: text.to_string(), // button text
            hovered: false, // hover state
            action: act, //checks if specific button was pressed or not
            luxary: false,
            count: 0,
        }
    }

    //draws the button onto the screen
    pub fn draw(&self) {
        //draws button and highlighted button
        let highlight = format!("{}_highlight", &self.text);
        match self.hovered {
            true => sprite!(&highlight, x = self.hitbox.0 - 1, y = self.hitbox.1 - 1),
            false => sprite!(&self.text, x = self.hitbox.0, y = self.hitbox.1)
        };
    }

    //summons pipi (draw function only used by pipi)
    pub fn summon(&mut self) {
        
        let anim = animation::get("PIPI");
        if self.hovered {
            anim.use_sprite("PIPI#WAVE");
            anim.set_repeat(1);
            self.count += 1;
        }
        
        if self.count > 5 {
            anim.use_sprite("PIPI#FLIP_good");
            anim.set_repeat(1);
            self.count = 0;
        }

        sprite!(
            animation_key = "PIPI",
            default_sprite = "PIPI#HAPPY_good", x = self.hitbox.0, y = self.hitbox.1
        );


        // match self.hovered {
        //     true => sprite!("PIPI#WAVE", x = self.hitbox.0, y = self.hitbox.1),
        //     false => sprite!("PIPI#HAPPY_good", x = self.hitbox.0, y = self.hitbox.1)
        // };
    }
    
    //checks if the mouse is hovering the button or not
    pub fn check(&mut self, mut select: (i32,i32)) -> i32{
        //gets the mouses world space position (its x and y on screen)
        let m = pointer();
        let(mx, my) = m.xy();
        //gets gamepad player 1
        let gp = gamepad(0);

        if self.hover(self.hitbox, mx, my) {
            if self.hitbox.1 > 30 { //checks if the button selected is Pipi
                if m.just_pressed(){
                    self.action = true; // Call function local to button
                    return self.hitbox.0;
                }else {
                    return self.hitbox.0;
                }
            } else {
                return select.0;
            }
            
        } 
        if self.hover(self.hitbox, select.0, select.1) {
            // Check if button is pressed (press z)
            if gp.a.just_pressed(){
                self.action = true; // Call function local to button
                return self.hitbox.0;
            }else {
                return self.hitbox.0;
            }
        } else {
            return select.0;
        }
        //made copy of if statement to check if selected is hovering
        
    }

    pub fn hover(&mut self, hitbox: (i32, i32, i32, i32), mx: i32, my: i32) -> bool {
        if mx >= hitbox.0 && mx <= hitbox.0 + hitbox.2
        && my >= hitbox.1 && my <= hitbox.1 + hitbox.3 {
            self.hovered = true;
            return true;
        } else {
            self.hovered = false;
            return false;
        }
    }

}
