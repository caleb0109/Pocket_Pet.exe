use std::fmt::format;

use crate::BorshDeserialize;
use crate::BorshSerialize;
use turbo::prelude::*;




#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Player{
    pub due_date: i32,
    pub day: i32,
    pub account: i32,
    pub salary: i32,
    pub activity: i32,
    pub affection: i32,
    pub affectionmax: i32,
    pub hunger: i32, 
    pub cleanliness: i32,
    pub playanim: [bool; 5],
}

impl Player {
    pub fn new()-> Self {
        Self{
            due_date: 14,
            day: 0,
            account: 4,
            salary: 3,
            activity: 3,
            affection: 0,
            affectionmax: 10,
            hunger: 5,
            cleanliness: 5,
            playanim: [false, false, false, false, false],
        }
    }

    pub fn active_check(&self) -> bool {
        if self.activity >= 1{
            return true;
        } else {
            return false;
        }
    }
    
    pub fn feed(&mut self, luxary: bool){
        let mut cost = 1;
        if self.active_check() {
            if luxary {
                cost = 2;
                self.hunger += 5;
            }
            if self.account >= cost {
                self.account -= cost;
                self.hunger += 3;
                self.cleanliness -=1;
                self.activity -= 1;
                self.playanim[0] = true;
            }
            
        }
    }

    pub fn shower(&mut self, luxary: bool){
        let mut cost = 1;
        if self.active_check() {
            if luxary {
                cost = 2;
                self.cleanliness += 5;
            }
            if self.account >= cost {
                self.account -= cost;
                self.cleanliness += 3;
                self.hunger -= 1;
                self.activity -= 1;
                self.playanim[1] = true;
            }           

        }
    }

    pub fn working(&mut self){
        let cap = 5;
        if self.active_check() {
            self.account += self.salary;
            self.hunger -= 1;
            self.cleanliness -=1;
            self.activity -= 1;
            if self.account > cap {
                self.account = 5;
            }
            self.playanim[2] = true;
        } else {
            return;
        }
    }

    pub fn go_sleep(&mut self){
        self.playanim[4] = true;
        self.hunger -= 1;
        self.cleanliness -=1;
        self.activity = 3;
        self.day += 1;       
    }

    pub fn allowance(&mut self) {
        let cost = 2;
        if self.active_check() {
            if self.account >= cost {
                self.account -= cost;
                self.hunger -= 1;
                self.cleanliness -=1;
                if self.affection < self.affectionmax {
                   self.affection += 1; 
                }               
                self.activity -= 1;

                self.playanim[3] = true;
            }               
        }       
    } 
        
}