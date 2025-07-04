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
        pipi: ActionButton,
        food: ActionButton,
        shower: ActionButton,
        work: ActionButton,
        allowance: ActionButton,
        sleep: ActionButton,
        sm: ActionButton,
        main: ActionButton,
        player: Player,
        select: (i32,i32),
        frame: u32,
        tweens:HashMap<String, Tween<f32>>,
        cameraPos: (i32,i32)

    } = Self {
        screen: Scene::Main,
        pipi: ActionButton::new("PIPI",(330, 30, 60, 69),false),
        food: ActionButton::new("food",(304, 114, 34, 34),false),
        shower: ActionButton::new("shower", (343, 114, 34, 34),false),
        work: ActionButton::new("work", (265, 114, 34, 34),false),
        allowance: ActionButton::new("allowance", (382, 114, 34, 34),false),
        sleep: ActionButton::new("sleep", (421, 114, 34, 34),false),
        sm: ActionButton::new("sm", (240, 60, 20, 20), false),
        main: ActionButton::new("main", (210, 60, 20, 20), false),
        player: Player::new(),
        select: (265,114),
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

    //gets mouse
    //sets the select to the location that is being highlighted either by mouse or keyboard
    state.select.0 = state.pipi.check(state.select);
    state.select.0 = state.food.check(state.select);
    state.select.0 = state.shower.check(state.select);
    state.select.0 = state.work.check(state.select);
    state.select.0 = state.allowance.check(state.select);
    state.select.0 = state.sleep.check(state.select);
    state.select.0 = state.sm.check(state.select);
    state.select.0 = state.main.check(state.select);


    //gathers buttons to see if it was pressed or not
    let acted: [bool; 8] = [
        state.food.action,
        state.shower.action,
        state.work.action,
        state.allowance.action,
        state.sleep.action,
        state.pipi.action,
        state.sm.action,
        state.main.action];


    //goes through for loop to see which button was pressed
    for n in 0..8 {
        if acted[n]{
            match n {
                0 => {
                    state.player.feed(state.food.luxury);
                    state.food.action = false;
                }
                1 => {
                    state.player.shower(state.shower.luxury);
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
                5 => {
                    state.pipi.action = false;
                }
                6 => {
                    state.cameraPos.0 = 120;
                    state.sm.action = false;
                }
                7 => {
                    state.cameraPos.0 = 360;
                    state.main.action = false;
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

//Screen
    sprite!("screen", x = 264, y = 19);
    let day = state.player.day.to_string();
    text!("Day {}", &day; x = 269, y = 105, color = 0x22406eff, font = "small");

//Summon Pipi
    state.pipi.summon();

    //Screen animations
    let anim = animation::get("screenanim");      
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
    sprite!(animation_key = "screenanim", x = 264, y = 19);

    // Draw
    state.food.draw();
    state.shower.draw();
    state.work.draw();
    state.allowance.draw();
    state.sleep.draw();
    state.sm.tempDraw();
    state.main.tempDraw();

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
    text!("Money: {:?}", state.player.account; x = 240, y = 0, color = 0x22406eff);
    text!("Activity: {:?}", state.player.activity; x = 240, y = 10, color = 0x22406eff);
    text!("Affection: {:?}", state.player.affection; x = 285, y = 0, color = 0x22406eff);
    text!("Day: {:?}", state.player.day; x = 450, y = 0, color = 0x22406eff);
    text!("Pipi count: {:?}", state.pipi.count; x = 415, y = 10, color = 0x22406eff);
    // Save GameState
    state.save();
});
