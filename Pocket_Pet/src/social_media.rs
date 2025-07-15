use turbo::*;
//need to discuss more about tween and sprite usage
#[turbo::serialize]
pub struct SocialMedia{
    pub posted: bool
}

impl SocialMedia {
    pub fn new() -> Self {
        Self {
            posted: false,
        }
    }

    //posts new posts depending on the requirements and returns a bool whether the post has been checked or not
    //dogshit code don't look too closely :)
    //it'll be more functional as i add more posts
    pub fn new_post(&mut self, unread: bool) -> bool {
        sprite!("icon", x = 38, y = 8);
        sprite!("post_intro", x = 58, y = 8);
        text!("PIPI: hey guys! your favorite", x = 62, y = 62, font = "FIVEPIXELS", color = 0x22406eff);
        text!("virtual pet, pipi is here!!!", x = 62, y = 68, font = "FIVEPIXELS", color = 0x22406eff);
    
        if unread == true {
            self.posted = true;
            return self.posted;
        }
        else {
            self.posted = false;
            return self.posted;
        }      
    }

    pub fn comment(&mut self) {
        
    }
}