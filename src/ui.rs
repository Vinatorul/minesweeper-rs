use piston_window::*;
use common::{ParamType, MoveDestination};

struct Block<'a> {
    name: &'a str, 
    num: u32,
    old_num: u32,
    char_id: char,
    selected: bool,
    min_val: u32,
    max_val: u32
}

impl<'a> Block<'a> {
    pub fn new(name: &'a str,
               num: u32,
               char_id: char,
               min_val: u32,
               max_val: u32) -> Block<'a>
    {
        Block {
            name: name,
            num: num,
            old_num: num,
            char_id: char_id,
            selected: false,
            min_val: min_val,
            max_val: max_val
        }
    }

    pub fn draw(&self,
                context: Context,
                graphics: &mut G2d,
                rect: &mut [u32; 4],
                glyps: &mut Glyphs)
    {
        let margin = 10;
        let text_height = 20;
        let block_height = 30;
        let text_padding = 7;
        let color = if self.selected {
            [0.0, 1.0, 0.0, 1.0]
        } else {
            [1.0, 1.0, 1.0, 1.0]
        };
        rect[1] += 20;
        rectangle(color,
                  [
                    (rect[0] + margin) as f64,
                    (rect[1]) as f64,
                    (rect[2] - 2*margin) as f64,
                    block_height as f64
                  ],
                  context.transform,
                  graphics);
        rect[1] += block_height;
        let transform = context.transform.trans((rect[0] + margin + text_padding) as f64,
                                                (rect[1] - text_padding) as f64);
        text::Text::colored([0.0, 0.0, 0.0, 1.0], text_height).draw(
                            &*format!("{}: {}", self.name, self.num),
                            glyps,
                            &context.draw_state,
                            transform,
                            graphics
                        );
        rect[1] += margin + text_height/2;
        let transform = context.transform.trans((rect[0] + margin) as f64,
                                                (rect[1]) as f64);
        let s = if self.selected {
            format!("Use arrows, press \"{}\" to apply changes", self.char_id)
        } else {
            format!("Press \"{}\" to change", self.char_id)
        };
        text::Text::colored([1.0, 1.0, 1.0, 1.0], text_height).draw(
                            &*s,
                            glyps,
                            &context.draw_state,
                            transform,
                            graphics
                        );
    }

    pub fn inc_safe(&mut self) {
        if self.num < self.max_val {
            self.num += 1;
        }
    }

    pub fn dec_safe(&mut self) {
        if self.num > self.min_val {
            self.num -= 1;
        }
    }

    pub fn apply_changes(&mut self) {
        self.old_num = self.num;
    }

    pub fn discard_changes(&mut self) {
        self.num = self.old_num;
    }
}

pub struct UI<'a> {
    blocks: Vec<Block<'a>>,
    selected_block: i32
}    

impl<'a> UI<'a> {
    pub fn new(height: u32, width: u32, mines: u32) -> UI<'a> {
        UI {
            blocks: vec![Block::new("Field width", width, 'W', 5, 50),
                         Block::new("Field height", height, 'H', 5, 50),
                         Block::new("Mines", mines, 'M', 1, 2500),
            ],
            selected_block: -1
        }
    }

    pub fn draw(&mut self,
            context: Context,
            graphics: &mut G2d,
            mut rect: [u32; 4],
            glyps: &mut Glyphs)
    {
        for b in self.blocks.iter() {
            b.draw(context, graphics, &mut rect, glyps);
        }
    }

    pub fn proc_key(&mut self, block: ParamType) -> Option<u32> {
        let c = match block {
            ParamType::Width => 'W',
            ParamType::Height => 'H',
            ParamType::Mines => 'M'
        };
        if self.selected_block >= 0 {
            let selected = self.blocks.get_mut(self.selected_block as usize).unwrap();
            if selected.char_id == c {
                selected.selected = false;
                selected.apply_changes();
                self.selected_block = -1;
                return Some(selected.num);
            }
        }
        for i in 0..self.blocks.len() {
            let item = self.blocks.get_mut(i).unwrap();
            let b = item.char_id == c;
            item.selected = b;
            if b {
                self.selected_block = i as i32;
            } else {
                item.discard_changes();
            }
        }
        None
    }

    pub fn change_selected(&mut self, dest: MoveDestination) {
        assert!(self.selected_block >= 0);
        let selected = self.blocks.get_mut(self.selected_block as usize).unwrap();
        match dest {
            MoveDestination::Up | MoveDestination::Right => selected.inc_safe(),
            MoveDestination::Down | MoveDestination::Left => selected.dec_safe()
        }
    }
}