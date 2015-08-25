use rand;
use rand::Rng;
use std::collections::VecDeque;

pub enum Content {
    Number(u8),
    Bomb,
    None
}

struct Cell {
    index: u32,
    content: Content,
    revealed: bool
}

impl Cell {
    pub fn clear(&mut self) {
        self.content = Content::None;
        self.revealed = false;
    }   

    fn reveal(&mut self) {
        self.revealed = true;
    }
}

pub struct Field {
    cells: Vec<Cell>,
    width: u32,
    height: u32,
    mines: u32,
    size: u32
}

impl Field {
    pub fn new(width: u32, height: u32, mines: u32) -> Field {
        let mut field = Field {
            width: width,
            height: height,
            cells: vec![],
            mines: mines,
            size: width*height
        };
        for i in 0..field.size {
            field.cells.push(Cell{index: i, 
                           content: Content::None,
                           revealed: false});
        }
        field.fill();
        field
    }

    fn fill(&mut self) {
        self.clear();
        for _i in 0..self.mines {
            let ind = rand::thread_rng().gen_range(0, self.size);
            self.get_cell_mut(ind).content = Content::Bomb
        }
        let mut i: i32 = -1;
        let w = self.width as i32;
        while i < (self.size - 1) as i32 {
            i += 1;
            match self.get_content_safe(i) {
                Some(&Content::None) => {
                    let ct_bomb = |b| {
                        match b {
                            true => 1,
                            false => 0
                        }
                    };
                     // don`t care about row
                    let mut ct = ct_bomb(self.is_bomb_safe(i-w)) +
                                 ct_bomb(self.is_bomb_safe(i+w));
                    if i % w > 0 { // check left side position
                        ct += ct_bomb(self.is_bomb_safe(i-w-1)) +
                              ct_bomb(self.is_bomb_safe(i-1)) +
                              ct_bomb(self.is_bomb_safe(i+w-1));
                    }
                    if i % w < w - 1 { // check right side position
                        ct += ct_bomb(self.is_bomb_safe(i-w+1)) +
                              ct_bomb(self.is_bomb_safe(i+1)) +
                              ct_bomb(self.is_bomb_safe(i+w+1));
                    }
                    if ct > 0 {
                        self.get_cell_mut(i as u32).content = Content::Number(ct);
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
    }

    pub fn reveal(&mut self, i: u32) -> &Content {
        let cell = self.get_cell_mut(i);
        cell.reveal();
        &cell.content
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

    fn is_bomb_safe(&self, i: i32) -> bool {
        match self.get_content_safe(i) {
            Some(&Content::Bomb) => true,
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
}