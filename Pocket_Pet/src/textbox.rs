use turbo::{text::Text, *};
static SCRIPT_PATH: &str = std::include_str!("script");

#[turbo::serialize]

pub struct TextBox {
    pub lines: Vec<String>,
    pub current_line: usize,
    pub speaking: bool,
    pub animdone: bool,
    pub cgshowing: bool
}
impl TextBox {
    pub fn new() -> Self {
        Self {
            lines: SCRIPT_PATH.split("\r\n").map(|line| line.to_string()).collect(),
            current_line: 0,
            speaking: false,
            animdone: true,
            cgshowing: false
        }
    }

    pub fn changeDay(&mut self, day: i32,) {
        match day {
            1 => {  
                let n = self.lines.iter().position(|line| line == "--day1");
                self.current_line = n.unwrap_or(0) + 1;
                self.speaking = true;
            }
            2 => {
                let n = self.lines.iter().position(|line| line == "--day2");
                self.current_line = n.unwrap_or(0) + 1;
                self.speaking = true;
            }
            6 => {
                let n = self.lines.iter().position(|line| line == "--day6");
                self.current_line = n.unwrap_or(0) + 1;
                self.speaking = true;
            }
            14 => {
                let n = self.lines.iter().position(|line| line == "--day14");
                self.current_line = n.unwrap_or(0) + 1;
                self.speaking = true;
                audio::play("pipiEvil");
                audio::set_volume("pipiEvil", 0.1);
            }
            _ => {}
        }       
    }

    pub fn drawText(&mut self, time: usize) {        
        let text;

        if self.animdone == false {
            text = "";
        } else {
            text = &self.lines[self.current_line];
        }
        
            
        if self.speaking == true {
            sprite!("speechbubble", x = 256, y= 114);
            text_box!{
                text,
                font = "FIVEPIXELS",
                color = 0xfae3deff,
                fixed = true,
                width = 200,
                height = 35,
                x =  21,
                y = 118,  
                //end = time/5         
            }
            self.pipiAnim();
            self.assessLine();
            self.pipiCG();
            
        }
    }

    pub fn assessLine(&mut self) {
        let m = pointer::world();
        if self.lines[self.current_line] == "--end" {
            self.speaking = false;
        } else if m.just_released() && self.pipiAnim() == false {
            self.current_line += 1;
            self.animdone = true; 
        } else if m.just_released() {
            self.current_line += 1;
        } else if self.pipiAnim() && self.animdone == false {
            self.current_line += 1;
            self.animdone = true;
        }
    }

    pub fn pipiAnim(&mut self) -> bool{
        let summon = animation::get("summon");
        summon.use_sprite("PIPI_summon");
        if self.animdone == true {
            summon.restart();
        }
        if self.lines[self.current_line] == "--pipisummon" {
            self.animdone = false;
            sprite!(animation_key = "summon", x = 320, y = 30);
            summon.set_repeat(1);
            summon.set_fill_forwards(true);
        }      
        //log!("{:?}", summon.done());
        return summon.done();    
    }

    //maybe draw full cg
    pub fn pipiCG(&mut self) {
        if self.lines[self.current_line] == "--cg" {
            self.cgshowing = true;
        } else if self.lines[self.current_line] == "--cgdone" {
            self.cgshowing = false;
        } 

        if self.cgshowing {
            sprite!("ending", x = 240, y = 0);
        } 
        
    }

    //affection max ending lines used when conditions met
    pub fn affectionMaxEnd(&mut self) {
        let n = self.lines.iter().position(|line| line == "--affectionmax");
        self.current_line = n.unwrap_or(0) + 1;
        self.speaking = true;
    }
}

