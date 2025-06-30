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
        player: Player,
        select: (i32,i32),
        toggle: bool,
        frame: u32,
        tweens:HashMap<String, Tween<f32>>

    } = Self {
        screen: Scene::Main,
        pipi: ActionButton::new("PIPI",(90, 30, 60, 69),false),
        food: ActionButton::new("food",(64, 114, 34, 34),false),
        shower: ActionButton::new("shower", (103, 114, 34, 34),false),
        work: ActionButton::new("work", (25, 114, 34, 34),false),
        allowance: ActionButton::new("allowance", (142, 114, 34, 34),false),
        sleep: ActionButton::new("sleep", (181, 114, 34, 34),false),
        sm: ActionButton::new("sm", (218, 60, 20, 20), false),
        player: Player::new(),
        select: (25,114),
        toggle: false,
        frame : 0,
        tweens: HashMap::from([
            ("social_media_change".to_string(), Tween::new(0.)),
            ("main_screen_change".to_string(), Tween::new(0.)),
        ])
    }
}

turbo::go!({
    let mut state = GameState::load();
    camera::set_xy(120, 80);
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
    state.select.0 = state.pipi.check(state.select);
    state.select.0 = state.food.check(state.select);
    state.select.0 = state.shower.check(state.select);
    state.select.0 = state.work.check(state.select);
    state.select.0 = state.allowance.check(state.select);
    state.select.0 = state.sleep.check(state.select);
    state.select.0 = state.sm.check(state.select);


    //gathers buttons to see if it was pressed or not
    let acted: [bool; 6] = [
        state.food.action,
        state.shower.action,
        state.work.action,
        state.allowance.action,
        state.sleep.action,
        state.pipi.action];


    //goes through for loop to see which button was pressed
    for n in 0..6 {
        if acted[n]{
            match n {
                0 => {
                    state.player.feed_or_shower(state.food.luxary);
                    state.food.action = false;
                }
                1 => {
                    state.player.feed_or_shower(state.shower.luxary);
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

//Summon Pipi
    state.pipi.summon();

// Draw
    state.food.draw();
    state.shower.draw();
    state.work.draw();
    state.allowance.draw();
    state.sleep.draw();
    state.sm.tempDraw();

    text!("Money: {:?}", state.player.account; x = 0, y = 0, color = 0x22406eff);
    text!("Activity: {:?}", state.player.activity; x = 0, y = 10, color = 0x22406eff);
    text!("Affection: {:?}", state.player.affection; x = 45, y = 0, color = 0x22406eff);
    text!("Day: {:?}", state.player.day; x = 200, y = 0, color = 0x22406eff);
    // Save GameState
    state.save();
});
