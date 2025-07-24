// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec
mod button;
mod player;
mod social_media;

use std::collections::HashMap;
use player::Player;
use button::button::ActionButton;
use social_media::SocialMedia;
use turbo::*;
use mrdirector;


#[turbo::game]
struct GameState{
    screen: u8,
    uibuttons: [ActionButton; 10],
    player: Player,
    sns: SocialMedia,
    unread: bool,
    select: (i32,i32),
    frame: u32,
    tweens:HashMap<String, Tween<f32>>,
    cameraPos: (i32,i32),
    comment: String,
} 


impl GameState {
    fn new() -> Self {
        Self {
            screen: 0,
            uibuttons: [
                ActionButton::new("food",(304, 117, 34, 34),false),
                ActionButton::new("shower", (343, 117, 34, 34),false),
                ActionButton::new("work", (265, 117, 34, 34),false),
                ActionButton::new("allowance", (382, 117, 34, 34),false),
                ActionButton::new("sleep", (421, 117, 34, 34),false),
                ActionButton::new("PIPI",(320, 30, 81, 69),false),
                ActionButton::new("sns", (243, 71, 19, 19), false),
                ActionButton::new("return", (218, 71, 19, 19), false),
                ActionButton::new("arrowup", (18, 125, 11, 14), false),
                ActionButton::new("arrowdown", (18, 141, 11, 14), false)
            ],
            player: Player::new(),
            sns: SocialMedia::new(),
            unread: true,
            select: (265,117),
            frame : 0,
            tweens: HashMap::from([
                ("social_media_change".to_string(), Tween::new(0.)),
                ("main_screen_change".to_string(), Tween::new(0.)),
            ]),
            cameraPos: (360, 80),
            comment: "".to_string(),
        }
    }
    pub fn update(&mut self) {

    if self.player.day == self.player.due_date {
        //self::new();
    }

    camera::set_xy(self.cameraPos.0,self.cameraPos.1);
    //checks if left or right has been inputted and if it has
    //then it moves the selected variable properly
    let gp = gamepad::get(0);
    if gp.left.just_pressed() {
        //makes sure that the select doesn't go off the buttonsto the far left
        if self.select.0 <= 265 {
            self.select.0 = 265;
        } else {
            self.select.0 -= 39; //why is it 39 pixel diff ;-;
        }
    }
    if gp.right.just_pressed() {
        //makes sure that the select doesn't go off the buttonsto the far right
        if self.select.0 >= 421 {
            self.select.0 = 421;
        } else {
            self.select.0 += 39;
        }
    }
    if gp.up.just_pressed() {
        self.select.0 = 243;
        if self.select.1 >= 71 {
            self.select.1 = 71;
        } else {
            self.select.1 -= 71;
        }
    }
    if gp.down.just_pressed() {
        self.select.0 = 265;
        if self.select.1 <= 0 {
            self.select.1 = 0;
        } else {
            self.select.1 += 71;
        }
    }

    //Background elements
    // match self.player.activity {
    //     3 => clear(0xfae3deff),
    //     2 => clear(0xc47a87ff),
    //     1 => clear(0x22406eff),
    //     _ => clear(0xfae3deff),
    // }
    clear(0xfae3deff);
    let frame = (self.frame as i32) / 2;
    for col in 0..27 {
        for row in 0..9 {
            let x = col * 18;
            let y = (row * 18 + frame) % (160);
            sprite!("dot", x = x, y = y);
        }
    }
    self.frame += 1;

//affection bar
//the values are hard coded, if you need to change the affectionmax, the sprites need to be changed as well
    sprite!("affectionbar", x = 267, y = 8);
    let increment = 160/self.player.affectionmax;
    let total = increment * self.player.affection;
    if self.player.affection == self.player.affectionmax {
        sprite!("barstart", x = 277, y = 10);
        sprite!("barmiddle", x = 285, y = 10, w = total);
        sprite!("barend", x = 444, y = 10);
    } else if  self.player.affection > 0 {
        sprite!("barstart", x = 277, y = 10);
        sprite!("barmiddle", x = 283, y = 10, w = total);
        sprite!("barend", x = 283 + total, y = 10);
    };
   

//Screen
    sprite!("screen", x = 264, y = 19);

    let day = self.player.day.to_string();
    text!("DAY {}", &day; x = 273, y = 105, color = 0x22406eff, font = "FIVEPIXELS");
    
    text!("TIME", x = 315, y = 105, color = 0x22406eff, font = "FIVEPIXELS");
    match self.player.activity {
        0 => sprite!("time#0", x = 340, y = 106),
        1 => sprite!("time#1", x = 340, y = 106),
        2 => sprite!("time#2", x = 340, y = 106),
        3 => sprite!("time#3", x = 340, y = 106),
        _ => sprite!("time#3", x = 340, y = 106),
    }

    text!("MONEY", x = 379, y = 105, color = 0x22406eff, font = "FIVEPIXELS");
    match self.player.account {
            0 => sprite!("money#0", x = 412, y = 106),
            1 => sprite!("money#1", x = 412, y = 106),
            2 => sprite!("money#2", x = 412, y = 106),
            3 => sprite!("money#3", x = 412, y = 106),
            4 => sprite!("money#4", x = 412, y = 106),
            5 => sprite!("money#5", x = 412, y = 106),
            _ => sprite!("money#5", x = 412, y = 106),
        }
        
//Summon Pipi
    self.uibuttons[5].summon(self.player.hunger, self.player.cleanliness);
    //log!("{:?}", self.player.hunger);

//Screen animations
    let anim = animation::get("screenanim");      
    //let mut play = false;
        for n in 0..5 {
            if self.player.playanim[n] {
                match n {
                    0 => {
                        anim.use_sprite("screen_anims#FEED");
                        anim.set_repeat(1);
                        self.player.playanim[0] = false;
                    }
                    1 => {
                        anim.use_sprite("screen_anims#SHOWER");
                        anim.set_repeat(1);
                        self.player.playanim[1] = false;
                    }
                    2 => {
                        anim.use_sprite("screen_anims#WORK");
                        anim.set_repeat(1);
                        self.player.playanim[2] = false;
                    }
                    3 => {
                        anim.use_sprite("screen_anims#ALLOWANCE");
                        anim.set_repeat(1);
                        self.player.playanim[3] = false;
                    }
                    4 => {
                        anim.use_sprite("screen_anims#SLEEP");
                        anim.set_repeat(1);
                        self.player.playanim[4] = false;
                    }
                    _ => {
                        anim.use_sprite("screen_anims#SLEEP");
                        anim.set_repeat(1);
                    }
                }
                
            }          
        }
    sprite!(animation_key = "screenanim", default_sprite = "screen_anims#empty", x = 264, y = 19);


    //sets the select to the location that is being highlighted either by mouse or keyboard
    //goes through for loop to see which button was pressed
    // Draw
    let can_click = anim.sprite_name() == "screen_anims#empty";
    
    for n in 0..10 {
        self.select.0 = self.uibuttons[n].check(self.select);
        if self.uibuttons[n].action && !can_click {
            self.uibuttons[n].action = false;
        }
        if self.uibuttons[n].action && can_click{
            //log!("{:?}", n);
            match n {
                0 => {
                    self.player.feed(self.uibuttons[0].luxury);
                    self.uibuttons[0].action = false;
                }
                1 => {
                    self.player.shower(self.uibuttons[1].luxury);
                    self.uibuttons[1].action = false;
                }
                2 => {
                    self.player.working();
                    self.uibuttons[2].action = false;
                }
                3 => {
                    self.player.allowance();
                    self.uibuttons[3].action = false;
                }
                4 => {
                    self.player.go_sleep();
                    self.uibuttons[4].action = false;
                }
                5 => {
                    self.uibuttons[5].action = false;
                }
                6 => {
                    self.cameraPos.0 = 120;
                    //will look more into this tween
                    // self.tweens.insert(
                    //     "social_media_change".to_string(),
                    //     Tween::new(360.).set(120.).duration(120).ease(Easing::EaseInOutSine)
                    // );
                    self.unread = false;
                    self.sns.cActive = false;
                    self.uibuttons[6].action = false;
                }
                7 => {
                    self.cameraPos.0 = 360;
                    for n in 0..self.sns.comments.len() {
                        if self.sns.comments[n].action {
                            self.sns.comments[n].action = false;
                        }
                    }
                    self.sns.cActive = false;
                    self.comment = "".to_string();
                    self.uibuttons[7].action = false;
                }
                8 => {
                    self.uibuttons[8].action = false;
                    self.cameraPos.1 -= 160;
                    self.uibuttons[8].hitbox.1 -=160;
                    self.uibuttons[9].hitbox.1 -=160;
                    if self.cameraPos.1 < 80 {
                        self.cameraPos.1 = 80;
                        self.uibuttons[8].hitbox.1 = 125;
                        self.uibuttons[9].hitbox.1 = 141;
                    }
                }
                9 => {
                    self.uibuttons[9].action = false;
                    self.cameraPos.1 += 160;
                    self.uibuttons[8].hitbox.1 +=160;
                    self.uibuttons[9].hitbox.1 +=160;
                }
                _ => {
                    text!("didn't work", x = 30, y = 40);
                }
            }
        }
        if n != 5{
            self.uibuttons[n].draw();
            self.uibuttons[6].sns_notif(self.unread);
        }
    }

    //Social Media UI
    sprite!("sns_bg", x = 32, y = 0);
    self.unread = self.sns.check_post(self.unread, self.player.hunger, self.player.cleanliness);
    self.sns.make_post();
    //self.sns.move_up();

    let mut currCom = 0;
    for n in 0..self.sns.comments.len() {
        if !self.sns.cActive{
            self.select.0 = self.sns.comments[n].check(self.select);
        }
        if self.sns.comments[n].action{
            self.sns.cActive = true;
            currCom = n;
            let keyboard = keyboard::get();
            
            // Append keyboard input to the buffer
            for c in keyboard.chars() {
                match c {
                    // Clear the buffer when Enter is pressed
                    '\n' => {
                        self.comment.clear();
                        self.sns.cActive = false;
                        self.sns.comments[n].action = false;
                    }

                    // Append all other chars to the buffer
                    ch => self.comment.push(ch),
                }
            }
 
            if keyboard.escape().just_pressed() {
                self.comment.clear();
                self.sns.cActive = false;
                self.sns.comments[n].action = false;
            }
            // Remove the last character when backspace is pressed
            if keyboard.backspace().just_pressed() {
                self.comment.pop();
            }
            text!("{:?}", self.comment; x = self.sns.comments[n].hitbox.0, y = self.sns.comments[n].hitbox.1, color = 0x22406eff, font = "FIVEPIXELS");
            text!("{:?}", currCom; x = 0, y = 10, color = 0x22406eff);
        }
        
    }

    
    //Stats
    //text!("Affection: {:?}", self.player.affection; x = 285, y = 0, color = 0x22406eff);
    //text!("hunger: {:?}", self.player.hunger; x = 430, y = 0, color = 0x22406eff, font = "FIVEPIXELS");
    //text!("Pipi count: {:?}", self.uibuttons[5].count; x = 415, y = 10, color = 0x22406eff);
    text!("Comment ACTIVE {:?}", self.sns.cActive; x = 0, y = 0, color = 0x22406eff);
    
    // Save GameState
    }
}


