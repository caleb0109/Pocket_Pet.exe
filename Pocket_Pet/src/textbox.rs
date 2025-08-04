use turbo::{text::Text, *};
static SCRIPT_PATH: &str = std::include_str!("script");

#[turbo::serialize]

pub struct TextBox {
    pub lines: Vec<String>,
    pub current_line: usize,
    pub spoken: [bool; 5],
    pub speaking: bool,
}
impl TextBox {
    pub fn new() -> Self {
        Self {
            lines: SCRIPT_PATH.split("\r\n").map(|line| line.to_string()).collect(),
            current_line: 0,
            spoken: [false, false, false, false, false],
            speaking: true
        }
    }

    pub fn changeDay(&mut self, day: i32,) {
        match day {
            1 => {  
                if self.spoken[0] == false {
                    let n = self.lines.iter().position(|line| line == "--day1");
                    self.current_line = n.unwrap_or(0) + 1;
                    self.speaking = true;
                    self.spoken[0] = true;
                    log!("{:?}", n);
                }                                 
            }
            2 => {
                if self.spoken[1] == false {
                    let n = self.lines.iter().position(|line| line == "--day2");
                    self.current_line = n.unwrap_or(0) + 1;
                    self.speaking = true;
                    self.spoken[1] = true;
                    log!("{:?}", n);
                }  
            }
            _ => {}
        }       
    }

    pub fn drawText(&mut self, time: usize) {        
        if self.speaking == true {
            sprite!("speechbubble", x = 256, y= 114);
            text_box!{
                &self.lines[self.current_line],
                font = "FIVEPIXELS",
                fixed = true,
                width = 200,
                height = 35,
                x =  21,
                y = 118,  
                end = time/5         
            }
            self.assessLine();
        }
        
    }

    pub fn assessLine(&mut self) {
        let m = pointer::world();
        if self.lines[self.current_line] == "--end" {
            self.speaking = false;
        } else if m.just_released() {
            self.current_line += 1;
        }

    }
}

