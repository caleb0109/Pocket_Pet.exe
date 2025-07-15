use turbo::*;
//need to discuss more about tween and sprite usage
#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct SocialMedia{
    pub posted: bool,
    pub ypos: Vec<u32>,
    pub triggered: [bool; 3]
}

impl SocialMedia {
    pub fn new() -> Self {
        Self {
            posted: false,
            ypos: vec![8],
            triggered: [false, false, false]
        }
    }

    //checks if the criteria for a new post has been fulfilled
    pub fn check_post(&mut self, unread: bool, hunger: i32, cleanliness: i32) -> bool {
        sprite!("icon", x = 38, y = 8);
        sprite!("sns_posts#intro", x = 58, y = self.ypos[0]);
        text!("PIPI: hey guys! your favorite", x = 63, y = 62, font = "FIVEPIXELS", color = 0xfae3deff);
        text!("virtual pet, pipi is here!!!", x = 63, y = 68, font = "FIVEPIXELS", color = 0xfae3deff);

        if hunger == 0 && !self.triggered[0] {
            self.move_post();
            self.triggered[0] = true;
            self.posted = true;
            log!("{:?}", self.triggered[0]);
            return self.posted;  
            
        }
        if cleanliness == 0 && !self.triggered[2] {
            self.move_post();
            let b = self.ypos.last().unwrap();
            sprite!("sns_posts#sample", x = 58, y = *b);
            self.posted = true;
            return self.posted;
        }

        if unread == true {
            self.posted = true;
            return self.posted;
        }
        else {
            self.posted = false;
            return self.posted;
        }      
    }

    //makes post depending on if the criteria was met
    pub fn make_post(&mut self) {
        for n in 0..3 {
            match n {
                0 => {
                    if self.triggered[0] == true {
                        let a = self.ypos.last().unwrap();
                        sprite!("sns_posts#hunger", x = 58, y = *a);
                        sprite!("icon", x = 38, y = *a);
                    }     
                }
                _ => {
                    //log!("fuck");
                }
            }
        }
    }

    //moves new posts down the feed
    pub fn move_post(&mut self) {
        let a = self.ypos.len();
        if a % 2 == 0 {
            let b = (a/2) * 73 + (a/2) * 87 + 8;
            self.ypos.push(b as u32);
            log!("{:?}", self.ypos);
            return;
        } else {
            let b = (a/2 + 1) * 73 + (a/2) * 87 + 8;
            self.ypos.push(b as u32);
            log!("{:?}", self.ypos);
            return;
        }
    }

    pub fn comment(&mut self) {
        
    }
}