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
        }
    }

    pub fn active_check(&self) -> bool {
        if self.activity >= 1{
            return true;
        } else {
            return false;
        }
    }
    
    pub fn feed_or_shower(&mut self, luxary: bool){
        let mut cost = 1;
        if self.active_check() {
            if luxary {
                cost = 2;
            }
            if self.account >= cost {
                self.account -= cost;
                self.activity -= 1;
            }

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
    }

    pub fn allowance(&mut self){
        let cost = 2;
        if self.active_check() {
            if self.account >= cost {
                self.account -= cost;
                self.affection += 10;
                self.activity -= 1;
            }
        }
    }
}