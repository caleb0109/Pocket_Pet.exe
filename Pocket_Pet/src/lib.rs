// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec
mod button;
mod player;
mod social_media;
mod textbox;

use std::{collections::HashMap, option, thread::AccessError};
use player::Player;
use button::button::ActionButton;
use social_media::SocialMedia;
use textbox::TextBox;
use turbo::time::tick;
use turbo::*;
use turbo::os::server::*;




#[turbo::game]
struct GameState{
    screen: u8,
    uibuttons: [ActionButton; 13],
    player: Player,
    sns: SocialMedia,
    textbox: TextBox,
    unread: bool,
    select: (i32,i32),
    frame: u32,
    tweens:HashMap<String, Tween<f32>>,
    cameraPos: (i32,i32),
    comment: String,
    allComments: Vec<String>,
    postID: usize,
    postPage: usize,
    timeStamp: usize,
    timepass: usize,
    introType: bool,
    repeatText: bool,
    upAnim: [bool; 3]
} 


impl GameState {
    pub fn new() -> Self {
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
                ActionButton::new("arrowdown", (18, 141, 11, 14), false),
                ActionButton::new("entercomment", (-196, 136, 156, 19), false),
                ActionButton::new("sns", (-22, 71, 19, 19), false),
                ActionButton::new("titlescreen_text", (316, 252, 156, 19), false)
            ],
            player: Player::new(),
            sns: SocialMedia::new(),
            textbox: TextBox::new(),
            unread: true,
            select: (265,117),
            frame : 0,
            tweens: HashMap::from([
                ("social_media_change".to_string(), Tween::new(0.)),
                ("main_screen_change".to_string(), Tween::new(0.)),
            ]),
            cameraPos: (360, 240),
            comment: "".to_string(),
            allComments: vec![],
            postID: 0,
            postPage: 0,
            timeStamp: time::tick(),
            timepass: 0,
            introType: false,
            repeatText: false,
            upAnim: [false, false, false],
        }
    }
    pub fn update(&mut self) {

    if !audio::is_playing("pipiDefault") {
        audio::play("pipiDefault");
        audio::set_volume("pipiDefault", 0.2);
    } 
    
    camera::set_xy(self.cameraPos.0,self.cameraPos.1);
    if self.cameraPos.0 == 120 { 
        self.select = (218, 71);
    }
    //checks if left or right has been inputted and if it has
    //then it moves the selected variable properly
    let gp = gamepad::get(0);
    if gp.left.just_pressed() {
        self.select.1 = 117;
        //makes sure that the select doesn't go off the buttonsto the far left
        if self.select.0 <= 265 {
            self.select.0 = 265;
        } else {
            self.select.0 -= 39; //why is it 39 pixel diff ;-;
        }
    }
    if gp.right.just_pressed() {
        self.select.1 = 117;
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
        if self.select.1 <= 117 {
            self.select.1 = 117;
        } else {
            self.select.1 += 71;
        }
    }

    //Background elements
    match self.player.activity {
        3 => clear(0xfae3deff),
        2 => clear(0xc47a87ff),
        1 => clear(0x22406eff),
        0 => clear(0x22406eff),
        _ => clear(0xfae3deff),
    }
    let frame = (self.frame as i32) / 2;
    for col in -13..27 {
        for row in 0..9 {
            let x = col * 18;
            let y = (row * 18 + frame) % (160);
            sprite!("dot", x = x, y = y);
        }
    }
    self.frame += 1;

    sprite!("sns_bg", x = 32, y = 0);
    sprite!("sns_bg", x = -208, y = 0);

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
    
    match self.player.affection {
        0 | 1  => sprite!("screen#1", x = 264, y = 19),
        2 | 3 => sprite!("screen#2", x = 264, y = 19),
        4 | 5 => sprite!("screen#3", x = 264, y = 19),
        6 | 7 => sprite!("screen#4", x = 264, y = 19),
        8 | 9 | 10 => sprite!("screen#5", x = 264, y = 19),
        _ => sprite!("screen#1", x = 264, y = 19),
    }

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
    if self.textbox.animdone == true {
        self.uibuttons[5].summon(self.player.hunger, self.player.cleanliness, self.player.day, self.player.activity, self.textbox.speaking, self.timepass);
    }

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
    let t = time::tick();  
    
    let can_click = anim.sprite_name() == "screen_anims#empty";   
    for n in 0..self.uibuttons.len() {
        self.select = self.uibuttons[n].check(self.select);
        if self.uibuttons[n].action && !can_click {
            self.uibuttons[n].action = false;
        }
        if n < 4 {
            if self.player.activity == 0 {
                self.uibuttons[n].action = false;
            }
        }
        if self.textbox.speaking == true {
            self.uibuttons[n].action = false;
        } 
        if self.player.day == 14 && t >= self.timeStamp + 5 {
            if self.textbox.speaking == false {
                self.player.day += 1;
            }
        }
        if self.player.affection == self.player.affectionmax && t >= self.timeStamp + 5 {
            if self.textbox.speaking == false {
                self.player.affection += 1;
            }
        }
        if self.uibuttons[n].action && can_click{
            self.timeStamp = time::tick() + 95;
            match n {
                0 => {
                    self.player.feed(self.uibuttons[0].luxury);
                    self.timepass = self.uibuttons[5].randomIdle();
                    self.uibuttons[0].action = false;
                    self.upAnim[0] = true;
                }
                1 => {
                    self.player.shower(self.uibuttons[1].luxury);
                    self.timepass = self.uibuttons[5].randomIdle();
                    self.uibuttons[1].action = false;
                    self.upAnim[1] = true;
                }
                2 => {
                    self.player.working();
                    self.timepass = self.uibuttons[5].randomIdle();
                    self.uibuttons[2].action = false;
                }
                3 => {
                    self.player.allowance();
                    self.timepass = self.uibuttons[5].randomIdle();
                    self.uibuttons[3].action = false;
                    self.upAnim[2] = true;
                }
                4 => {
                    self.player.go_sleep();
                    self.timepass = self.uibuttons[5].randomIdle();
                    self.uibuttons[4].action = false;
                    self.repeatText = false;
                    if self.player.day == 6 {
                        if self.player.affection > 7 {
                            self.player.affection -= 3;
                        } else if self.player.affection < 6 && self.player.affection > 2{
                            self.player.affection -= 1;
                        }
                    }
                }
                5 => {
                    self.uibuttons[5].action = false;
                }
                6 => {
                    self.cameraPos.0 = 120;
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
                    self.select = (243, 71);
                    self.uibuttons[7].action = false;
                }
                8 => {
                    self.uibuttons[8].action = false;
                    self.sns.arrowup();
                }
                9 => {
                    self.uibuttons[9].action = false;
                    self.sns.arrowdown();              
                }
                10 => {
                    self.sns.cActive = true;
                }
                11 => {
                    self.uibuttons[11].action = false;
                    self.cameraPos.0 = 120;
                    self.select = (218, 71);
                }
                12 => {
                    self.introType = true;
                }
                _ => {
                    text!("didn't work", x = 30, y = 40);
                }
            }
        }
        if n != 5 || n != 10 {
            if self.player.activity == 0 {
                if n < 4 {
                    self.uibuttons[n].nonselect();
                } else {
                    self.uibuttons[n].draw();
                }
            } else if self.player.account == 0 {
                if n < 4 && n != 2 {
                    self.uibuttons[n].nonselect();
                } else {
                    self.uibuttons[n].draw();
                }
            } else if self.player.account == 1 {
                if n == 3 {
                    self.uibuttons[3].nonselect();
                } else {
                    self.uibuttons[n].draw();
                }
            } else {
                self.uibuttons[n].draw();
            }
            
            self.uibuttons[6].sns_notif(self.unread);
        }
    }


    //status animations  
    let hungerup = animation::get("hungerup");
    let cleanlinessup = animation::get("cleanlinessup");
    let affectionup = animation::get("affectionup");
    //let upanim = animation::get("upanim");

    if self.upAnim[0] {        
        if t >= self.timeStamp + 10 {
            self.upAnim[1] = false;
            self.upAnim[2] = false;
            hungerup.use_sprite("hungerup");
            hungerup.set_repeat(1);
            hungerup.set_fill_forwards(true);
            if can_click && self.upAnim[1] == false && self.upAnim[2]  == false {
                sprite!(animation_key = "hungerup", default_sprite = "up_empty", x = 322, y = 43);
            }
        }
        if t == self.timeStamp + 72 {
            self.upAnim[0] = false;
        }
             
    }
    if self.upAnim[1] { 
        if t >= self.timeStamp + 19 {
            self.upAnim[0] = false;
            self.upAnim[2] = false;
            cleanlinessup.use_sprite("cleanlinessup");
            cleanlinessup.set_repeat(1);
            cleanlinessup.set_fill_forwards(true);
            if can_click && self.upAnim[0] == false && self.upAnim[2]  == false {
                sprite!(animation_key = "cleanlinessup", default_sprite = "up_empty", x = 322, y = 43);
            }
        }
        if t == self.timeStamp + 83 {
            self.upAnim[1] = false;
            
        }   
    }
    if self.upAnim[2] { 
        if t >= self.timeStamp + 42 {
            self.upAnim[0] = false;
            self.upAnim[1] = false;
            affectionup.use_sprite("affectionup");
            affectionup.set_repeat(1);
            affectionup.set_fill_forwards(true);
            if can_click && self.upAnim[0] == false && self.upAnim[1]  == false {
                sprite!(animation_key = "affectionup", default_sprite = "up_empty", x = 322, y = 43);
            }
            
        }
        if t == self.timeStamp + 100 {
            self.upAnim[2] = false;
            
        } 
    }

    


    //intro draw
    sprite!("titlescreen", x = 240, y = 160);
    self.uibuttons[12].draw();
    
    
    //pay deduction
    if self.player.hunger == 0 || self.player.cleanliness == 0 {
        self.player.salary = 1;
    } else {
        self.player.salary = 3;
    }

    //sets up correct event text
    if self.player.day != 1 {
        if can_click && t == self.timeStamp && !self.repeatText{
            self.textbox.changeDay(self.player.day);
            self.repeatText = true;
        }
    }
    
    //good ending text
    if self.player.affection == self.player.affectionmax && t == self.timeStamp + 4{
        self.textbox.affectionMaxEnd();
    }
    //draws text
    if self.player.day != 0 {
        self.textbox.drawText(t);
    }
    //Social Media UI
    self.unread = self.sns.check_post(self.unread, self.player.hunger, self.player.cleanliness, self.player.affection);
    self.sns.draw_page();

    for n in 0..self.sns.comments.len() {
        self.select = self.sns.comments[n].check(self.select);
        if self.sns.comments[n].action {
            self.cameraPos.0 = -120;
            match n {
                _ => {
                    let id: String = n.to_string();
                    self.postID = id.parse().unwrap();
                    self.sns.comments[n].action = false;
                }
            }
        }
    }

    for n in 0..self.sns.pages.len() {
        if self.sns.pages[n] {
            let id: String = n.to_string();
            self.postPage = id.parse().unwrap();
        }
    }

    let mut tracker: usize = 0;
    if self.postID == 0 {
        if self.postPage == 0 {
            tracker = 0;
        } else {
            tracker = self.postPage * 2;
        }
    }
    if self.postID == 2 {
        if self.postPage == 0 {
            tracker = 1;
        } else {
            tracker = (self.postPage * 2) + 1;
        }
    }

    for n in 0..self.sns.posts.len() {
        if n == tracker {
            if self.sns.posts[tracker] == "sns_posts#intro".to_string() {
                let commented = Comment::watch("firstComm").parse().unwrap_or(Comment { Comments: vec![] });
                self.allComments = commented.Comments.clone();
                sprite!("posts_text#intro", x = -183, y = 8);
            } else if self.sns.posts[tracker] == "sns_posts#hunger".to_string() {
                let commented = Comment::watch("hunger").parse().unwrap_or(Comment { Comments: vec![] });
                self.allComments = commented.Comments.clone();
                sprite!("posts_text#hungry", x = -183, y = 8);
            } else if self.sns.posts[tracker] == "sns_posts#dirty" {
                let commented = Comment::watch("clean").parse().unwrap_or(Comment { Comments: vec![] });
                self.allComments = commented.Comments.clone();
                 sprite!("posts_text#clean", x = -183, y = 8);
            } else if self.sns.posts[tracker] == "sns_posts#hunger_resolved" {
                let commented = Comment::watch("hungRes").parse().unwrap_or(Comment { Comments: vec![] });
                self.allComments = commented.Comments.clone();
                sprite!("posts_text#hunger_resolved", x = -183, y = 8);
            } else if self.sns.posts[tracker] == "sns_posts#dirty_resolved" {
                let commented = Comment::watch("cleanRes").parse().unwrap_or(Comment { Comments: vec![] });
                self.allComments = commented.Comments.clone();
                sprite!("posts_text#dirty_resolved", x = -183, y = 8);
            } else if self.sns.posts[tracker] == "sns_posts#money" {
                let commented = Comment::watch("money").parse().unwrap_or(Comment { Comments: vec![] });
                self.allComments = commented.Comments.clone();
                sprite!("posts_text#money", x = -183, y = 8);
            } else if self.sns.posts[tracker] == "sns_posts#gigachad" {
                let commented = Comment::watch("gigaChad").parse().unwrap_or(Comment { Comments: vec![] });
                self.allComments = commented.Comments.clone();
                sprite!("posts_text#gigachad", x = -183, y = 8);
            }
        }

    }
    
    if self.sns.cActive || self.introType{
        let keyboard = keyboard::get();
        
        // Append keyboard input to the buffer
        for c in keyboard.chars() {
            match c {
                // Clear the buffer when Enter is pressed
                '\n' => {
                    self.allComments.push(self.comment.to_string());
                    if self.introType {
                        self.uibuttons[12].action = false;
                        self.player.name = self.comment.clone();
                        self.introType = false;
                        self.cameraPos.1 = 80;
                        self.player.day += 1;
                        self.textbox.changeDay(self.player.day);
                        //self.allComments = vec![];
                    } else {
                        self.uibuttons[10].action = false;
                        self.sns.cActive = false;
                        let mut cmd = PostComment { 
                        ChangeComm: self.allComments.clone(), 
                        PostID: self.sns.posts.clone(), 
                        PostPage: self.postPage,
                        PostComm: self.postID};
                        cmd.exec();
                    }
                    self.comment.clear();
                }

                // Append all other chars to the buffer
                ch => {
                    if self.comment.len() == 25 {
                        self.comment.pop();
                    } else {
                        self.comment.push(ch);
                    }
                }
            }
        }
 
        if keyboard.escape().just_pressed() {
            self.comment.clear();
            self.allComments = vec![];
            if self.sns.cActive {
                self.sns.cActive = false;
                self.uibuttons[10].action =false;
                let cmd = Reset {
                    PostID: self.sns.posts.clone(),
                    PostPage: self.postPage,
                    PostComm: self.postID};
                cmd.exec();
            }
        }
        // Remove the last character when backspace is pressed
        if keyboard.backspace().just_pressed() {
            self.comment.pop();
        }
        if self.introType {
            text!("{}|", &self.comment; x = self.uibuttons[12].hitbox.0 + 2, y = self.uibuttons[12].hitbox.1 + 1, color = 0x22406eff, font = "FIVEPIXELS", opacity = if t % 32 < 16 { 1. } else { 0. });
            text!(&self.comment, x = self.uibuttons[12].hitbox.0 + 2, y = self.uibuttons[12].hitbox.1 + 1, color = 0x22406eff, font = "FIVEPIXELS");
        } else {
            text!("{}|", &self.comment; x = self.uibuttons[10].hitbox.0 + 2, y = self.uibuttons[10].hitbox.1 + 1, color = 0x22406eff, font = "FIVEPIXELS", opacity = if t % 32 < 16 { 1. } else { 0. });
            text!(&self.comment, x = self.uibuttons[10].hitbox.0 + 2, y = self.uibuttons[10].hitbox.1 + 1, color = 0x22406eff, font = "FIVEPIXELS");
        }
    }

    //Showing all comments in post
    let mut movingY = 27;
    for n in 0..self.allComments.len() {
        sprite!("commentbubble", x = -177, y = movingY);
        sprite!("otherplayericon", x = -189, y = movingY);
        text!(&self.allComments[n], x = -171, y = movingY + 1, color = 0x22406eff);
        movingY += 21;
    }

    sprite!("icon", x = -202, y = 8);

    if self.player.day >= self.player.due_date || self.player.affection > self.player.affectionmax{
        *self = GameState::new();
    }
    }
}


#[turbo::os::document(program = "comment")]
pub struct Comment {
    Comments: Vec<String>,
}
#[turbo::os::command(program = "comment", name = "addComm")]
pub struct PostComment {
    ChangeComm: Vec<String>,
    PostID: Vec<String>,
    PostPage: usize,
    PostComm: usize,
}
impl CommandHandler for PostComment {
    fn run(&mut self, user_id: &str) -> Result<(), std::io::Error> {
        //reads files for each comment section
        let mut firstComm = fs::read("firstComm").unwrap_or(Comment {Comments: vec![]});
        let mut hungerComm = fs::read("hunger").unwrap_or(Comment{Comments: vec![]});
        let mut cleanComm = fs::read("clean").unwrap_or(Comment{Comments: vec![]});
        let mut hungResComm = fs::read("hungRes").unwrap_or(Comment{Comments: vec![]});
        let mut cleanResComm = fs::read("cleanRes").unwrap_or(Comment{Comments: vec![]});
        let mut moneyComm = fs::read("money").unwrap_or(Comment{Comments: vec![]});
        let mut gigaComm = fs::read("gigaChad").unwrap_or(Comment{Comments: vec![]});

        //tracks which comment section was accessed
        let mut tracker: usize = 0;

        //if the first comment button on whatever page was pressed,
        //it will multiply the page by 2, but if the page is the first page
        //then that means its the first post
        if self.PostComm == 0 {
            if self.PostPage == 0 {
                tracker = 0;
            } else {
                tracker = self.PostPage * 2;
            }
        }
        //if the second comment button on whatever page was pressed,
        //it will multiply the page by 2 and add 1, but if the page is the first page
        //then that means its the second post
        if self.PostComm == 2 {
            if self.PostPage == 0 {
                tracker = 1;
            } else {
                tracker = (self.PostPage * 2) + 1;
            }
        }

        //for loop to check which post's comment section was pressed
        for n in 0..self.PostID.len() {
            if n == tracker {
                if self.PostID[tracker] == "sns_posts#intro" {
                    firstComm.Comments = self.fileRead(firstComm.Comments.clone()).clone();
                }
                if self.PostID[tracker] == "sns_posts#hunger" {
                    hungerComm.Comments = self.fileRead(hungerComm.Comments.clone());
                }
                if self.PostID[tracker] == "sns_posts#dirty" {
                    cleanComm.Comments = self.fileRead(cleanComm.Comments.clone());
                }
                if self.PostID[tracker] == "sns_posts#hunger_resolved" {
                    hungResComm.Comments = self.fileRead(hungResComm.Comments.clone());
                }
                if self.PostID[tracker] == "sns_posts#dirty_resolved" {
                    cleanResComm.Comments = self.fileRead(cleanResComm.Comments.clone());
                }
                if self.PostID[tracker] == "sns_posts#money" {
                    moneyComm.Comments = self.fileRead(moneyComm.Comments.clone());
                }
                if self.PostID[tracker] == "sns_posts#gigachad" {
                    gigaComm.Comments = self.fileRead(gigaComm.Comments.clone());
                }
            }
        }
        
        log!("{:?}", tracker);
        log!("{:?}", firstComm);
        log!("{:?}", hungerComm);
        log!("{:?}", cleanComm);
        log!("{:?}", hungResComm);
        log!("{:?}", cleanResComm);
        log!("{:?}", moneyComm);
        log!("{:?}", gigaComm);
        //writes to all files with most recent data
        fs::write("firstComm", &firstComm.Comments)?;
        fs::write("hunger", &hungerComm.Comments)?;
        fs::write("clean", &cleanComm.Comments)?;
        fs::write("hungRes", &hungResComm.Comments)?;
        fs::write("cleanRes", &cleanResComm.Comments)?;
        fs::write("money", &moneyComm.Comments)?;
        fs::write("gigaChad", &gigaComm.Comments)?;
        Ok(())
    }
    
}

impl PostComment {
    fn fileRead (&mut self, mut currComment: Vec<String>) -> Vec<String>{
        if currComment.len() >= 5 {
            currComment.remove(0);
        }
        currComment.push(self.ChangeComm[self.ChangeComm.len()-1].clone());
        return currComment;
    }
}
#[turbo::os::command(program = "comment", name = "reset")]
pub struct Reset {
    PostID: Vec<String>,
    PostPage: usize,
    PostComm: usize,
}
impl CommandHandler for Reset {
    fn run(&mut self, user_id: &str) -> Result<(), std::io::Error> {
        let mut firstComm = fs::read("firstComm").unwrap_or(Comment {Comments: vec![]});
        let mut hungerComm = fs::read("hunger").unwrap_or(Comment{Comments: vec![]});
        let mut cleanComm = fs::read("clean").unwrap_or(Comment{Comments: vec![]});
        let mut hungResComm = fs::read("hungRes").unwrap_or(Comment{Comments: vec![]});
        let mut cleanResComm = fs::read("cleanRes").unwrap_or(Comment{Comments: vec![]});
        let mut moneyComm = fs::read("money").unwrap_or(Comment{Comments: vec![]});
        let mut gigaComm = fs::read("gigaChad").unwrap_or(Comment{Comments: vec![]});
        
        let mut tracker: usize = 0;
        if self.PostComm == 0 {
            if self.PostPage == 0 {
                tracker = 0;
            } else {
                tracker = self.PostPage * 2;
            }
        }
        if self.PostComm == 2 {
            if self.PostPage == 0 {
                tracker = 1;
            } else {
                tracker = (self.PostPage * 2) + 1;
            }
        }

        for n in 0..self.PostID.len() {
            if n == tracker {
                if self.PostID[tracker] == "sns_posts#intro" {
                    firstComm.Comments = vec![];
                }
                if self.PostID[tracker] == "sns_posts#hunger" {
                    hungerComm.Comments = vec![];
                }
                if self.PostID[tracker] == "sns_posts#clean" {
                    cleanComm.Comments = vec![];
                }
                if self.PostID[tracker] == "sns_posts#hunger_resolved" {
                    hungResComm.Comments = vec![];
                }
                if self.PostID[tracker] == "sns_posts#dirty_resolved" {
                    cleanResComm.Comments = vec![];
                }
                if self.PostID[tracker] == "sns_posts#money" {
                    moneyComm.Comments = vec![];
                }
                if self.PostID[tracker] == "sns_posts#gigachad" {
                    gigaComm.Comments = vec![];
                }
            }
        }

        fs::write("firstComm", &firstComm.Comments)?;
        fs::write("hunger", &hungerComm.Comments)?;
        fs::write("clean", &cleanComm.Comments)?;
        fs::write("hungRes", &hungResComm.Comments)?;
        fs::write("cleanRes", &cleanResComm.Comments)?;
        fs::write("money", &moneyComm.Comments)?;
        fs::write("gigaChad", &gigaComm.Comments)?;
        Ok(())
    }
}
