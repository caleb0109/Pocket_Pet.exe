// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec

use std::string;

turbo::init!{
    struct GameState{
        // screen: enum Scene {
        //     Main,
        //     Social,
        // },
        food: UIButton,
        shower: UIButton,
        work: UIButton,
        allowance: UIButton,
        sleep: UIButton,
        due_date: u32,
        day: u32,
        account: i32,
        salary: u32,
        luxary: bool,
        activity: u32,
        toggle: bool,

    } = Self {
        //screen: Main,
        food: UIButton::new("Give Food",(200, 120, 20, 20),false),
        shower: UIButton::new("Give Shower", (160, 120, 20, 20),false),
        work: UIButton::new("Go to Work", (120, 120, 20, 20),false),
        allowance: UIButton::new("Give Money", (80, 120, 20, 20),false),
        sleep: UIButton::new("Go to Sleep", (40, 120, 20, 20),false),
        due_date: 14,
        day: 0,
        account: 0,
        salary: 3,
        luxary: false,
        activity: 3,
        toggle: false,
    }
}

turbo::go!({
    let mut state = GameState::load();

    //gets mouse
    //checks 
    state.food.check();
    state.shower.check();
    state.work.check();
    state.allowance.check();
    state.sleep.check();

    let acted: [bool; 5] = [
        state.food.action,
        state.shower.action,
        state.work.action,
        state.allowance.action,
        state.sleep.action];

    for n in 0..5 {
        if acted[n]{
            match n {
                0 => {
                    text!("food", x = 20, y = 40);
                }
                1 => {
                    text!("shower", x = 20, y = 40);
                }
                2 => {
                    text!("work", x = 20, y = 40);
                }
                3 => {
                    text!("allowance", x = 20, y = 40);
                }
                4 => {
                    text!("sleep", x = 20, y = 40);
                }
                _ => {
                    text!("didn't work", x = 30, y = 40);
                }
            }
        }
    }
    
    // Draw
    state.food.draw();
    state.shower.draw();
    state.work.draw();
    state.allowance.draw();
    state.sleep.draw();

    // Save GameState
    state.save();
});

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct UIButton {
    pub hitbox: (i32, i32, i32, i32),
    pub text: String,
    pub hovered: bool,
    pub count: u32,
    pub action: bool,
}

impl UIButton {
    pub fn new (text: &str, hitbox: (i32, i32, i32, i32), act: bool) -> Self {
        Self {
            hitbox, // x, y, w, h
            text: text.to_string(), // button text
            hovered: false, // hover state
            count: 0, // checking if click works or not
            action: act,
        }
    }

    //draws the button onto the screen
    pub fn draw(&self) {
        // sets the color of the button
        let (c1, c2): (u32, u32) = match self.hovered {
            true => (0x323b42ff, 0xffffffff),
            false => (0xffffffff, 0x323b42ff)
        };
        // Calculate text offset for centering onto button
        let (x, y) = 
            (self.hitbox.0 + (self.hitbox.2/2) - (self.text.len() as f32 * 2.5) as i32, 
            self.hitbox.1 + (self.hitbox.3/2) - 3);

        // Draw button
        rect!(x = self.hitbox.0, y = self.hitbox.1, w = self.hitbox.2, h = self.hitbox.3, color = c1);
        // Draw text
        text!(&self.text, x = x, y = y, color = c2);
        text!("{:?}", self.count; x = x + 25, y = y - 20, color = c2);
    }
    
    //checks if the mouse is hovering the button or not
    pub fn check(&mut self) {
        //gets the mouses world space position (its x and y on screen)
        let m = pointer();
        let(mx, my) = m.xy();
        let gp = gamepad(0);
        if let Some(b) = self.hover(self.hitbox, mx, my) {
            // Check if mouse clicked on button
            if m.just_pressed()||gp.a.just_pressed(){
                b.click(); // Call function local to button
            }
        }
    }


}

pub trait Clickable {
    // checks if mouse is actually hovering over the button or not
    fn hover(&mut self, hitbox: (i32, i32, i32, i32), mx: i32, my: i32) -> Option<&mut Self> {
        if mx >= hitbox.0 && mx <= hitbox.0 + hitbox.2
        && my >= hitbox.1 && my <= hitbox.1 + hitbox.3 {
            Clickable::hover_state(self, true);
            return Some(self)
        } else {
            Clickable::hover_state(self, false);
            return None
        }
    }

    // Private function for toggling hover state
    fn hover_state(&mut self, hover: bool) {}

    // Private function for registering clicks on button
    fn click(&mut self) {}
}

// Implement Clickable for UIButton and override private functionality
impl Clickable for UIButton {
    // Toggle hover state
    fn hover_state(&mut self, hover: bool) {
        self.hovered = hover; 
    }

    //counts click
    fn click(&mut self) {
        self.count += 1;
        self.action = true;
    }
}