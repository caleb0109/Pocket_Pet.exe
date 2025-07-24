use std::ptr::null;

use turbo::*;


#[turbo::serialize]
pub struct ActionButton {
    pub hitbox: (i32, i32, i32, i32),
    pub text: String,
    pub hovered: bool,
    pub action: bool,
    pub luxury: bool,
    pub count: u32,
}

impl ActionButton {
    pub fn new (text: &str, hitbox: (i32, i32, i32, i32), act: bool) -> Self {
        Self {
            hitbox, // x, y, w, h
            text: text.to_string(), // button text
            hovered: false, // hover state
            action: act, //checks if specific button was pressed or not
            luxury: false,
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

     pub fn tempDraw(&self) {
        // Color references
        let (c1, c2): (u32, u32) = match self.hovered {
            true => (0x323b42ff, 0xffffffff),
            false => (0xffffffff, 0x323b42ff)
        };
        // Calculate text offset for centering
        let (x, y) = 
            (self.hitbox.0 + (self.hitbox.2/2) - (self.text.len() as f32 * 2.5) as i32, 
            self.hitbox.1 + (self.hitbox.3/2) - 3);

        // Draw button
        rect!(x = self.hitbox.0, y = self.hitbox.1, w = self.hitbox.2, h = self.hitbox.3, color = c1);
    }

    //summons pipi (draw function only used by pipi)
    pub fn summon(&mut self, hunger: u32, cleanliness: u32) {
        let selected = self.pipiselect();
        let anim = animation::get("PIPI");

        if selected{
            anim.use_sprite("PIPI#WAVE");
            anim.set_repeat(1);               
            self.count += 1;   
        }
        
        if self.count > 5 {
            anim.use_sprite("PIPI#FLIP_good");
            anim.set_repeat(1);
            self.count = 0;
        }

        if hunger <= 1 && cleanliness <= 1{
            anim.use_sprite("PIPI#HAPPY_hungrydirty");
        } else if hunger <= 1 {
            anim.use_sprite("PIPI#HAPPY_hungry");
        } else if cleanliness <= 1 {
            anim.use_sprite("PIPI#HAPPY_dirty");
        } else {
            anim.use_sprite("PIPI#HAPPY_good");
        }
        

        sprite!(
            animation_key = "PIPI",
            default_sprite = "PIPI#HAPPY_good", x = self.hitbox.0, y = self.hitbox.1
        );
        
    }

    //checks if mouse clicked pipi
    pub fn pipiselect(&mut self) -> bool {
        let m = pointer::world();
        let (mx, my) = m.xy();
        if self.hover(self.hitbox, mx, my) {
            if m.just_pressed() {
                self.action = true;
                return true;
            }else {
                return false;
            }
        }else {
            return false;
        }
    }

    pub fn sns_notif(&mut self, newpost: bool) {
        if newpost {
            self.text = "sns_notif".to_string();
            self.hitbox.0 = 241;
            self.hitbox.1 = 69;
        } else {
            self.text = "sns".to_string();
            self.hitbox.0 = 243;
            self.hitbox.1 = 71;
        }
    }
    
    //checks if the mouse is hovering the button or not
    pub fn check(&mut self, mut select: (i32,i32)) -> i32{
        //gets the mouses world space position (its x and y on screen)
        let m = pointer::world();
        let(mx, my) = m.xy();
        //gets gamepad player 1
        let gp = gamepad::get(0);

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

