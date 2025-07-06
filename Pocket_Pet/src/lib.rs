// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec
mod button;
mod player;
mod social_media;

use std::collections::HashMap;
use player::Player;
use button::ActionButton;
use turbo::prelude::*;


turbo::init!{
    struct GameState{
        screen: enum Scene {
            Main,
            Social,
        },
        uibuttons: [ActionButton; 8],
        player: Player,
        select: (i32,i32),
        frame: u32,
        tweens:HashMap<String, Tween<f32>>,
        cameraPos: (i32,i32)

    } = Self {
        screen: Scene::Main,
        uibuttons: [
            ActionButton::new("food",(304, 117, 34, 34),false),
            ActionButton::new("shower", (343, 117, 34, 34),false),
            ActionButton::new("work", (265, 117, 34, 34),false),
            ActionButton::new("allowance", (382, 117, 34, 34),false),
            ActionButton::new("sleep", (421, 117, 34, 34),false),
            ActionButton::new("PIPI",(330, 30, 60, 69),false),
            ActionButton::new("sns", (243, 71, 19, 19), false),
            ActionButton::new("main", (210, 60, 20, 20), false),
        ],
        player: Player::new(),
        select: (265,117),
        frame : 0,
        tweens: HashMap::from([
            ("social_media_change".to_string(), Tween::new(0.)),
            ("main_screen_change".to_string(), Tween::new(0.)),
        ]),
        cameraPos: (360, 80)
    }
}

turbo::go!({
    let mut state = GameState::load();
    camera::set_xy(state.cameraPos.0,state.cameraPos.1);
    //checks if left or right has been inputted and if it has
    //then it moves the selected variable properly
    let gp = gamepad(0);
    if gp.left.just_pressed() {
        //makes sure that the select doesn't go off the buttonsto the far left
        if state.select.0 <= 265 {
            state.select.0 = 265;
        } else {
            state.select.0 -= 39; //why is it 39 pixel diff ;-;
        }
    }
    if gp.right.just_pressed() {
        //makes sure that the select doesn't go off the buttonsto the far right
        if state.select.0 >= 421 {
            state.select.0 = 421
        } else {
            state.select.0 += 39;
        }
    }

    //sets the select to the location that is being highlighted either by mouse or keyboard

    for n in 0..8 {
        state.select.0 = state.uibuttons[n].check(state.select);
    }


    //goes through for loop to see which button was pressed
    for n in 0..8 {
        if state.uibuttons[n].action{
            match n {
                0 => {
                    state.player.feed(state.uibuttons[0].luxury);
                    state.uibuttons[0].action = false;
                }
                1 => {
                    state.player.shower(state.uibuttons[1].luxury);
                    state.uibuttons[1].action = false;
                }
                2 => {
                    state.player.working();
                    state.uibuttons[2].action = false;
                }
                3 => {
                    state.player.allowance();
                    state.uibuttons[3].action = false;
                }
                4 => {
                    state.player.go_sleep();
                    state.uibuttons[4].action = false;
                }
                5 => {
                    state.uibuttons[5].action = false;
                }
                6 => {
                    state.cameraPos.0 = 120;
                    state.uibuttons[6].action = false;
                }
                7 => {
                    state.cameraPos.0 = 360;
                    state.uibuttons[7].action = false;
                }
                _ => {
                    text!("didn't work", x = 30, y = 40);
                }
            }
        }
    } 
    
       
    //Background elements
    // match state.player.activity {
    //     3 => clear(0xfae3deff),
    //     2 => clear(0xc47a87ff),
    //     1 => clear(0x22406eff),
    //     _ => clear(0xfae3deff),
    // }
    clear(0xfae3deff);
    let frame = (state.frame as i32) / 2;
    for col in 14..27 {
        for row in 0..9 {
            let x = col * 18;
            let y = (row * 18 + frame) % (160);
            sprite!("dot", x = x, y = y);
        }
    }
    state.frame += 1;

//affection bar
    sprite!("affectionbar", x = 267, y = 8);

//Screen
    sprite!("screen", x = 264, y = 19);

    let day = state.player.day.to_string();
    text!("DAY {}", &day; x = 269, y = 106, color = 0x22406eff, font = "small");
    
    text!("TIME", x = 310, y = 106, color = 0x22406eff, font = "small");
    match state.player.activity {
        0 => sprite!("time#0", x = 335, y = 106),
        1 => sprite!("time#1", x = 335, y = 106),
        2 => sprite!("time#2", x = 335, y = 106),
        3 => sprite!("time#3", x = 335, y = 106),
        _ => sprite!("time#3", x = 335, y = 106),
    }

    text!("MONEY", x = 360, y = 106, color = 0x22406eff, font = "small");
    match state.player.account {
            0 => sprite!("money#0", x = 390, y = 106),
            1 => sprite!("money#1", x = 390, y = 106),
            2 => sprite!("money#2", x = 390, y = 106),
            3 => sprite!("money#3", x = 390, y = 106),
            4 => sprite!("money#4", x = 390, y = 106),
            5 => sprite!("money#5", x = 390, y = 106),
            _ => sprite!("money#5", x = 390, y = 106),
        }
        
//Summon Pipi
    state.uibuttons[5].summon();

//Screen animations
    let anim = animation::get("screenanim");      
    let mut play = false;
        for n in 0..5 {
            if state.player.playanim[n] {
                match n {
                    0 => {
                        anim.use_sprite("screen_anims#FEED");
                        anim.set_repeat(1);
                        state.player.playanim[0] = false;
                    }
                    1 => {
                        anim.use_sprite("screen_anims#SHOWER");
                        anim.set_repeat(1);
                        state.player.playanim[1] = false;
                    }
                    2 => {
                        anim.use_sprite("screen_anims#WORK");
                        anim.set_repeat(1);
                        state.player.playanim[2] = false;
                    }
                    3 => {
                        anim.use_sprite("screen_anims#ALLOWANCE");
                        anim.set_repeat(1);
                        state.player.playanim[3] = false;
                    }
                    4 => {
                        anim.use_sprite("screen_anims#SLEEP");
                        anim.set_repeat(1);
                        state.player.playanim[4] = false;
                    }
                    _ => {
                        anim.use_sprite("screen_anims#SLEEP");
                        anim.set_repeat(1);
                    }
                }
                
            }          
        }
        // let workanim = animation::get("screen_work");
        // workanim.set_repeat(1);
    // if play{
        
    //     sprite!("screen_work", x = 264, y = 19);
    //     //play = false;
    // }
    //sprite!("screen_work", x = 264, y = 19);
    sprite!(animation_key = "screenanim", default_sprite = "screen_anims#empty", x = 264, y = 19);

    // Draw
    for n in 0..8 {
        if n != 5{
            state.uibuttons[n].draw();
        }
    }

    state.uibuttons[7].tempDraw();

    //Social Media UI
    path!(
        start = (195,0),
        end = (195,160),
        size = 2,
    );

    circ!(
        d = 18,
        x = 10,
        y = 10,
        color = 0xfae3deff,
        border_size = 2,
        border_color = 0xffffffff,
    );
    rect!(
        w = 160,
        h = 70,
        x = 30,
        y = 10,
        color = 0xfae3deff,
        border_size = 2 ,
        border_color = 0xffffffff,
    );
    rect!(
        w = 120,
        h = 20,
        x = 30,
        y = 80,
        color = 0xfae3deff,
        border_size = 2 ,
        border_color = 0xffffffff,
    );

    //Stats
    text!("Anim: {:?}", state.player.playanim; x = 240, y = 0, color = 0x22406eff);
    //text!("Affection: {:?}", state.player.affection; x = 285, y = 0, color = 0x22406eff);
    text!("Day: {:?}", state.player.day; x = 450, y = 0, color = 0x22406eff);
    text!("Pipi count: {:?}", state.uibuttons[5].count; x = 415, y = 10, color = 0x22406eff);
    // Save GameState
    state.save();
});
