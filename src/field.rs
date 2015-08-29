use rand;
use rand::Rng;
use std::collections::VecDeque;
use piston_window::*;

pub enum Content {
    Number(u8),
    Mine,
    None
}

pub enum MoveDestination {
    Up,
    Down,
    Left,
    Right
}

struct Cell {
    content: Content,
    revealed: bool
}

impl Cell {
    pub fn clear(&mut self) {
        self.content = Content::None;
        self.revealed = false;
    }   

    fn reveal(&mut self) -> &Content {
        self.revealed = true;
        &self.content
    }
}

pub struct Field {
    cells: Vec<Cell>,
    width: u32,
    height: u32,
    mines: u32,
    size: u32,
    selected_x: u32,
    selected_y: u32,
    nubmers_total: u32,
    nubmers_opened: u32
}

impl Field {
    pub fn new(width: u32, height: u32, mines: u32) -> Field {
        let mut field = Field {
            width: width,
            height: height,
            cells: vec![],
            mines: mines,
            size: width*height,
            selected_x: width/2,
            selected_y: height/2,
            nubmers_total: 0,
            nubmers_opened: 0
        };
        for _i in 0..field.size {
            field.cells.push(Cell{content: Content::None,
                                  revealed: false});
        }
        field.fill();
        field
    }

    fn fill(&mut self) {
        self.clear();
        for _i in 0..self.mines {
            let ind = rand::thread_rng().gen_range(0, self.size);
            self.get_cell_mut(ind).content = Content::Mine
        }
        let mut i: i32 = -1;
        let w = self.width as i32;
        while i < (self.size - 1) as i32 {
            i += 1;
            match self.get_content_safe(i) {
                Some(&Content::None) => {
                    let ct_mine = |b| {
                        match b {
                            true => 1,
                            false => 0
                        }
                    };
                     // don`t care about row
                    let mut ct = ct_mine(self.is_mine_safe(i-w)) +
                                 ct_mine(self.is_mine_safe(i+w));
                    if i % w > 0 { // check left side position
                        ct += ct_mine(self.is_mine_safe(i-w-1)) +
                              ct_mine(self.is_mine_safe(i-1)) +
                              ct_mine(self.is_mine_safe(i+w-1));
                    }
                    if i % w < w - 1 { // check right side position
                        ct += ct_mine(self.is_mine_safe(i-w+1)) +
                              ct_mine(self.is_mine_safe(i+1)) +
                              ct_mine(self.is_mine_safe(i+w+1));
                    }
                    if ct > 0 {
                        self.get_cell_mut(i as u32).content = Content::Number(ct);
                        self.nubmers_total += 1;
                    }
                },
                _ => {}
            }
        }
    }

    fn clear(&mut self) {
        for i in 0..self.size {
            self.get_cell_mut(i).clear();
        }
        self.nubmers_opened = 0;
        self.nubmers_total = 0;
    }

    pub fn reveal(&mut self, i: u32) -> &Content {
        if !self.revealed(i) {
            if let &Content::Number(_i) = self.get_cell_mut(i).reveal() {
                self.nubmers_opened += 1;
            }
        }
        &self.get_content(i)
    }

    pub fn revealed(&self, i: u32) -> bool {
        self.get_cell(i).revealed
    }

    fn get_cell_mut(&mut self, i:u32) -> &mut Cell {
        self.cells.get_mut(i as usize)
            .unwrap_or_else(|| panic!("Range check error at Field::get_cell_mut ({})", i))
    }

    fn get_cell(& self, i:u32) -> &Cell {
        self.cells.get(i as usize)
            .unwrap_or_else(|| panic!("Range check error at Field::get_cell ({})", i))
    }

    fn get_content_safe(&self, i:i32) -> Option<&Content> {
        if (i < 0) || ((i as u32) >= self.size) {
            None
        } else {
            Some(self.get_content(i as u32))
        }
    }

    pub fn get_content(& self, i: u32) -> &Content {
        &self.get_cell(i).content
    }

    fn is_mine_safe(&self, i: i32) -> bool {
        match self.get_content_safe(i) {
            Some(&Content::Mine) => true,
            _ => false
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn restart(&mut self) {
        self.fill();
    } 

    pub fn reveal_all(&mut self) {
        for i in 0..self.size {
            self.get_cell_mut(i).revealed = true;
        }  
    }

    pub fn chain_reveal(&mut self, u: u32) {
        let w = self.width as i32;
        // clojure to check for blank cells
        let mut check = |x, d: &mut VecDeque<i32>| {
            match self.get_content_safe(x) {
                Some(&Content::None) => {
                    if !self.revealed(x as u32) {
                        d.push_back(x);
                    }
                    self.reveal(x as u32);
                },
                Some(&Content::Number(_n)) => {
                    self.reveal(x as u32);
                },
                _ => {}
            }
        };
        // BFS initialize
        let deq = &mut VecDeque::new();
        deq.push_back(u as i32);
        // BFS
        while !deq.is_empty() {
            let i = deq.pop_front().unwrap();
            // don`t care about row
            check(i-w, deq);
            check(i+w, deq);
            if i % w > 0 { // check left side position
                check(i-w-1, deq);
                check(i-1, deq);
                check(i+w-1, deq);
            }
            if i % w < w - 1 { // check right side position
                check(i-w+1, deq);
                check(i+1, deq);
                check(i+w+1, deq);
            }
        }
    }

    pub fn draw(&mut self,
            context: Context,
            graphics: &mut G2d,
            field_rect: [u32; 4],
            glyps: &mut Glyphs)
    {
        let cell_w = field_rect[2] / self.get_width();
        let cell_h = field_rect[3] / self.get_height();
        for i in 0..self.get_width() {
            for j in 0..self.get_height() {
                if !self.revealed(i + j*self.get_width()) {
                    continue;
                }
                match *self.get_content(i + j*self.get_width()) {
                    Content::Mine => {
                        rectangle([1.0, 0.0, 0.0, 1.0],
                                  [
                                    (field_rect[0] + i*cell_w) as f64,
                                    (field_rect[1] + j*cell_h) as f64,
                                    cell_w as f64,
                                    cell_h as f64
                                  ],
                                  context.transform,
                                  graphics);
                    },
                    Content::Number(n) => {
                        let transform = context.transform.trans((field_rect[0] + i*cell_w) as f64 + 5.0,
                                                                (field_rect[1] + (j+1)*cell_h) as f64 - 5.0);
                        text::Text::colored([1.0, 1.0, 1.0, 1.0], cell_h).draw(
                            &*n.to_string(),
                            glyps,
                            &context.draw_state,
                            transform,
                            graphics
                        );
                    },
                    Content::None => {
                        rectangle([1.0, 1.0, 1.0, 1.0],
                                  [
                                    (field_rect[0] + i*cell_w) as f64,
                                    (field_rect[1] + j*cell_h) as f64,
                                    cell_w as f64,
                                    cell_h as f64
                                  ],
                                  context.transform,
                                  graphics);
                    }
                }
            }
        }
        rectangle([0.5, 0.5, 0.5, 0.75],
                  [
                    (field_rect[0] + self.selected_x*cell_w) as f64,
                    (field_rect[1] + self.selected_y*cell_h) as f64,
                    cell_w as f64,
                    cell_h as f64
                  ],
                  context.transform,
                  graphics);
        for i in 0..self.get_width()+1 {
            line::Line::new([0.5, 0.5, 0.5, 1.0], 1.0)
                .draw([
                        (field_rect[0] + i*cell_w) as f64,
                        field_rect[1] as f64,
                        (field_rect[0] + i*cell_w) as f64,
                        (field_rect[1] + field_rect[3]) as f64
                      ],
                      &context.draw_state,
                      context.transform,
                      graphics);
            line::Line::new([0.5, 0.5, 0.5, 1.0], 1.0)
                .draw([
                        field_rect[0] as f64,
                        (field_rect[1] + i*cell_h) as f64,
                        (field_rect[0] + field_rect[2]) as f64,
                        (field_rect[1] + i*cell_h) as f64
                      ],
                      &context.draw_state,
                      context.transform,
                      graphics);
        }
    }

    pub fn move_selection(&mut self, dest: MoveDestination) {
        match dest {
            MoveDestination::Up => {
                if self.selected_y > 0 {
                    self.selected_y -= 1;
                }
            },
            MoveDestination::Down => {
                if self.selected_y < self.height - 1 {
                    self.selected_y += 1;
                }
            },
            MoveDestination::Left => {
                if self.selected_x > 0 {
                    self.selected_x -= 1;
                }
            },
            MoveDestination::Right => {
                if self.selected_x < self.width - 1 {
                    self.selected_x += 1;
                }
            }
        }
    }

    pub fn get_selected_ind(&self) -> u32 {
        self.selected_x + self.selected_y*self.width
    }

    pub fn is_victory(&self) -> bool {
        self.nubmers_total == self.nubmers_opened
    }
}