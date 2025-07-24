use turbo::{serde_json::to_string, *};
//need to discuss more about tween and sprite usage
#[turbo::serialize]
pub struct SocialMedia{
    pub posted: bool,
    pub ypos: Vec<u32>,
    pub posts: Vec<String>,
    pub pages: Vec<bool>,
    pub triggered: [bool; 3],
    pub cActive: bool,
}

impl SocialMedia {
    pub fn new() -> Self {
        Self {
            posted: false,
            ypos: vec![8],
            posts: vec!["sns_posts#intro".to_string()],
            pages: vec![true],
            triggered: [false, false, false],
            cActive: false,
        }
    }

    //checks if the criteria for a new post has been fulfilled
    pub fn check_post(&mut self, unread: bool, hunger: u32, cleanliness: u32) -> bool {

        if hunger == 0 && !self.triggered[0] {              
            self.posts.insert(0, "sns_posts#hunger".to_string());
            self.posted = true; 
            self.triggered[0] = true;
            return self.posted;
            
        }
        
        if cleanliness == 0 && !self.triggered[1] {           
            self.posts.insert(0, "sns_posts#clean".to_string());
            self.posted = true;
            self.triggered[1] = true;
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

    //draws posts on each page depending on the page number and number of posts
    pub fn draw_posts(&mut self, pagenum: usize) {
        //only one post exists, aka the intro post
        if self.posts.len() == 1{
            sprite!(&self.posts[0], x = 58, y = 8); 
            self.draw_misc(1);
        //the last page of an odd number of posts; there is only one post on this page  
        } else if self.posts.len() % 2 == 1 && pagenum == self.pages.len() - 1 {
            sprite!(&self.posts[pagenum * 2], x = 58, y = 8);
            self.draw_misc(1);
        //pages with even number of posts, or the first and middle pages of odd number posts
        }else {
            sprite!(&self.posts[pagenum*2], x = 58, y = 8);
            sprite!(&self.posts[pagenum*2 + 1], x = 58, y = 81);
            self.draw_misc(2);
        }

    }

    //checks which page the sns is currently on and draws it
    pub fn draw_page(& mut self) {
        let postslength = self.posts.len();
        let pageslength = self.pages.len();

        //increases number of pages depending on number of posts
        if postslength/2 == pageslength && postslength % 2 == 1{
            self.pages.push(false);
        } else if postslength/2 > pageslength {
            self.pages.push(false);
        } 
        
        //calls draw post function depending on which page player is currently on
        for n in 0..self.pages.len() {
            if self.pages[n] == true {
                self.draw_posts(n);
            }   
        }
    }

    //draw icon and other buttons
    pub fn draw_misc(&mut self, posts: u32) {
        if posts == 1 {
            sprite!("icon", x = 38, y = 8);
        } else {
            sprite!("icon", x = 38, y = 8);
            sprite!("icon", x = 38, y = 81);
        }
    }

    //called when pressing down arrow, moves which element in page vector is true
    pub fn arrowdown(& mut self) -> bool {
        let mut position = self.pages.iter().position(|x| *x == true).unwrap();
        let mut selectable = true;
        if self.pages.len() - 1 > position {
            self.pages[position] = false;
            position += 1;
            self.pages[position] = true;
            return selectable;

        } else {
            self.pages[position] = true;
            selectable = false;
            return selectable;

        }
    }

    //called when pressing up arrow, moves which element in page vector is true
    //pages[0] is the top, pressing the down arrow is equal to increasing the index of the page vector
    pub fn arrowup(& mut self) -> bool {
        let mut position = self.pages.iter().position(|x| *x == true). unwrap();
        let mut selectable = true;
        if position == 0 {
            self.pages[position] = true;
            selectable = false;
            return selectable;
        } else {
            self.pages[position] = false;
            position -= 1;
            self.pages[position] = true;
            return selectable;
        }
    }

}
