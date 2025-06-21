// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec

turbo::init!{
    struct GameState{
        // screen: enum Scene {
        //     Main,
        //     Social,
        // },
        food: ActionButton,
        shower: ActionButton,
        work: ActionButton,
        allowance: ActionButton,
        sleep: ActionButton,
        player: PlayerAction,
        select: (i32,i32),
        toggle: bool,
        frame: u32

    } = Self {
        //screen: Main,
        food: ActionButton::new("food",(64, 114, 34, 34),false),
        shower: ActionButton::new("shower", (103, 114, 34, 34),false),
        work: ActionButton::new("work", (25, 114, 34, 34),false),
        allowance: ActionButton::new("allowance", (142, 114, 34, 34),false),
        sleep: ActionButton::new("sleep", (181, 114, 34, 34),false),
        player: PlayerAction::new(),
        select: (25,114),
        toggle: false,
        frame : 0
    }
}

turbo::go!({
    let mut state = GameState::load();
    
    //checks if left or right has been inputted and if it has
    //then it moves the selected variable properly
    let gp = gamepad(0);
    if gp.left.just_pressed() {
        //makes sure that the select doesn't go off the buttonsto the far left
        if state.select.0 <= 25 {
            state.select.0 = 25;
        } else {
            state.select.0 -= 39; //why is it 39 pixel diff ;-;
        }
    }
    if gp.right.just_pressed() {
        //makes sure that the select doesn't go off the buttonsto the far right
        if state.select.0 >= 181 {
            state.select.0 = 181
        } else {
            state.select.0 += 39;
        }
    }

    
    //gets mouse
    //sets the select to the location that is being highlighted either by mouse or keyboard
    //i'll look into cleaning this up tomorrow
    state.select.0 = state.food.check(state.select);
    state.select.0 = state.shower.check(state.select);
    state.select.0 = state.work.check(state.select);
    state.select.0 = state.allowance.check(state.select);
    state.select.0 = state.sleep.check(state.select);



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
       
//Background elements
    clear(0xfae3deff);
    let frame = (state.frame as i32) / 2;
    for col in 0..16 {
        for row in 0..9 {
            let x = col * 18;
            let y = (row * 18 + frame) % (160);
            sprite!("dot", x = x, y = y);
        }
    }
    state.frame += 1;

//Screen
    sprite!("screen", x = 24, y = 19);
    let day = state.player.day.to_string();
    text!("Day {}", &day; x = 29, y = 105, color = 0x22406eff, font = "small");

// Draw
    state.food.draw();
    state.shower.draw();
    state.work.draw();
    state.allowance.draw();
    state.sleep.draw();

    text!("Money: {:?}", state.player.account; x = 0, y = 0, color = 0x22406eff);
    text!("Activity: {:?}", state.player.activity; x = 0, y = 10, color = 0x22406eff);
    text!("Affection: {:?}", state.player.affection; x = 45, y = 0, color = 0x22406eff);
    text!("Day: {:?}", state.player.day; x = 200, y = 0, color = 0x22406eff);
    // Save GameState
    state.save();
});

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ActionButton {
    pub hitbox: (i32, i32, i32, i32),
    pub text: String,
    pub hovered: bool,
    pub action: bool,
    pub luxary: bool,
}

impl ActionButton {
    pub fn new (text: &str, hitbox: (i32, i32, i32, i32), act: bool) -> Self {
        Self {
            hitbox, // x, y, w, h
            text: text.to_string(), // button text
            hovered: false, // hover state
            action: act, //checks if specific button was pressed or not
            luxary: false,
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
    
    //checks if the mouse is hovering the button or not
    pub fn check(&mut self, mut select: (i32,i32)) -> i32{
        //gets the mouses world space position (its x and y on screen)
        let m = pointer();
        let(mx, my) = m.xy();
        //gets gamepad player 1
        let gp = gamepad(0);

        if let Some(b) = self.hover(self.hitbox, mx, my) {
            if m.just_pressed(){
                b.click(); // Call function local to button
                return b.hitbox.0;
            }else {
                return b.hitbox.0;
            }
        } 
        if let Some(b) = self.hover(self.hitbox, select.0, select.1) {
            // Check if button is pressed (press z)
            if gp.a.just_pressed(){
                b.click(); // Call function local to button
                return b.hitbox.0;
            }else {
                return b.hitbox.0;
            }
        } else {
            return select.0;
        }
        //made copy of if statement to check if selected is hovering
        
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
impl Clickable for ActionButton {
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
    
    pub fn feed_or_shower(&mut self, button: &ActionButton, identify: &str){
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