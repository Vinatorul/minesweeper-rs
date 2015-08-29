use piston_window::*;

pub enum BlockType {
    Width,
    Height,
    Mines
}

struct Block<'a> {
    name: &'a str, 
    num: u32,
    hotkey: char,
    selected: bool
}

impl<'a> Block<'a> {
    pub fn new(name: &'a str,
               num: u32,
               hotkey: char) -> Block<'a>
    {
        Block {
            name: name,
            num: num,
            hotkey: hotkey,
            selected: false
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
            format!("Use arrows, press \"{}\" to apply changes", self.hotkey)
        } else {
            format!("Press \"{}\" to change", self.hotkey)
        };
        text::Text::colored([1.0, 1.0, 1.0, 1.0], text_height).draw(
                            &*s,
                            glyps,
                            &context.draw_state,
                            transform,
                            graphics
                        );
    }
}

pub struct UI<'a> {
    blocks: Vec<Block<'a>>
}    

impl<'a> UI<'a> {
    pub fn new(height: u32, width: u32, mines: u32) -> UI<'a> {
        UI {
            blocks: vec![Block::new("Field width", width, 'W'),
                         Block::new("Field height", height, 'H'),
                         Block::new("Mines", mines, 'M'),
            ]    
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

    pub fn select(&mut self, block: BlockType) {
        let c = match block {
            BlockType::Width => 'W',
            BlockType::Height => 'H',
            BlockType::Mines => 'M'
        };
        for i in 0..self.blocks.len() {
            let b = self.blocks.get(i).unwrap().hotkey == c;
            self.blocks.get_mut(i).unwrap().selected = b;
        }
    }
}