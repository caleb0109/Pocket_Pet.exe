// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec

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
        player: PlayerAction,
        toggle: bool,

    } = Self {
        //screen: Main,
        food: UIButton::new("Give Food",(200, 120, 20, 20),false),
        shower: UIButton::new("Give Shower", (160, 120, 20, 20),false),
        work: UIButton::new("Go to Work", (120, 120, 20, 20),false),
        allowance: UIButton::new("Give Money", (80, 120, 20, 20),false),
        sleep: UIButton::new("Go to Sleep", (40, 120, 20, 20),false),
        player: PlayerAction::new(),
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
                    state.player.feed_or_shower(&state.food, "food");
                    state.food.action = false;
                }
                1 => {
                    state.player.feed_or_shower(&state.shower, "shower");
                    state.shower.action = false;
                }
                2 => {
                    state.player.working();
                    state.work.action = false;
                }
                3 => {
                    state.player.allowance();
                    state.allowance.action = false;
                }
                4 => {
                    state.player.go_sleep();
                    state.sleep.action = false;
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

    text!("Money: {:?}", state.player.account; x = 0, y = 0);
    text!("Activity: {:?}", state.player.activity; x = 0, y = 10);
    text!("Affection: {:?}", state.player.affection; x = 45, y = 0);
    text!("Day: {:?}", state.player.day; x = 200, y = 0);
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
    pub luxary: bool,
}

impl UIButton {
    pub fn new (text: &str, hitbox: (i32, i32, i32, i32), act: bool) -> Self {
        Self {
            hitbox, // x, y, w, h
            text: text.to_string(), // button text
            hovered: false, // hover state
            count: 0, // checking if click works or not
            action: act, //checks if specific button was pressed or not
            luxary: false,
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
    }
    
    //checks if the mouse is hovering the button or not
    pub fn check(&mut self) {
        //gets the mouses world space position (its x and y on screen)
        let m = pointer();
        let(mx, my) = m.xy();
        //gets gamepad player 1
        let gp = gamepad(0);
        if let Some(b) = self.hover(self.hitbox, mx, my) {
            // Check if mouse clicked on button or is z is pressed
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
        //turns action true
        self.action = true;
    }
}

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct PlayerAction{
    pub due_date: i32,
    pub day: i32,
    pub account: i32,
    pub salary: i32,
    pub activity: i32,
    pub affection: i32,
}

impl PlayerAction {
    pub fn new()-> Self {
        Self{
            due_date: 14,
            day: 0,
            account: 4,
            salary: 3,
            activity: 3,
            affection: 0,
        }
    }

    pub fn active_check(&self) -> bool {
        if self.activity >= 1{
            return true;
        } else {
            text!("Don't have enough activity points", x = 10, y= 20);
            return false;
        }
    }
    
    pub fn feed_or_shower(&mut self, button: &UIButton, identify: &str){
        let mut cost = 1;
        if self.active_check() {
            if button.luxary {
                cost = 2;
            }
            if self.account >= cost {
                self.account -= cost;
                self.activity -= 1;
                if identify == "food" {
                    text!("Yummy!", x = 10, y= 20);
                } else if identify == "shower" {
                    text!("So clean!", x = 10, y= 20);
                }
            } else {
                text!("Don't have enough money", x = 10, y= 20);
                return;
            }

        } else {
            return;
        }
    }

    pub fn working(&mut self){
        let cap = 5;
        if self.active_check() {
            self.account += self.salary;
            self.activity -= 1;
            text!("WORKING", x = 10, y= 20);
            if self.account > cap {
                self.account = 5;
            }
        } else {
            return;
        }
    }

    pub fn go_sleep(&mut self){
        self.activity = 3;
        self.day += 1;
        text!("eepy time!", x = 10, y= 20);
    }

    pub fn allowance(&mut self){
        let cost = 2;
        if self.active_check() {
            if self.account >= cost {
                self.account -= cost;
                self.affection += 10;
                self.activity -= 1;
                text!("Thanks for the allowance!", x = 10, y= 20);
            } else {
                text!("Don't have enough money", x = 10, y= 20);
                return;
            }
        } else {
            return;
        }
        
    }
}