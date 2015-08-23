use rand;
use rand::Rng;

pub enum Content {
    Number(u8),
    Bomb,
    None
}

pub struct Cell {
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
    pub width: u32,
    pub height: u32,
    mines: u32
}

impl Field {
    pub fn new(width: u32, height: u32, mines: u32) -> Field {
        let mut field = Field {
            width: width,
            height: height,
            cells: vec![],
            mines: mines
        };
        for i in 0..width*height {
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
            let ind = rand::thread_rng().gen_range(0, self.width*self.height);
            self.get_cell_mut(ind).content = Content::Bomb
        }
        for i in 0..self.width {
            for j in 0..self.height {           
                match self.get_cell(i*self.width + j).content {
                    Content::None => {
                        let mut ct : u8 = 0;
                        let i1 = i as i32;
                        let j1 = j as i32;
                        if let Some(b) = self.is_bomb_safe(i1-1, j1-1) { 
                            ct += b;
                        }   
                        if let Some(b) = self.is_bomb_safe(i1-1, j1) {
                            ct += b;
                        } 
                        if let Some(b) = self.is_bomb_safe(i1-1, j1+1) {
                            ct += b;
                        } 
                        if let Some(b) = self.is_bomb_safe(i1, j1-1) {
                            ct += b;
                        } 
                        if let Some(b) = self.is_bomb_safe(i1, j1+1) {
                            ct += b;
                        } 
                        if let Some(b) = self.is_bomb_safe(i1+1, j1-1) {
                            ct += b;
                        } 
                        if let Some(b) = self.is_bomb_safe(i1+1, j1) {
                            ct += b;
                        } 
                        if let Some(b) = self.is_bomb_safe(i1+1, j1+1) {
                            ct += b;
                        } 
                        if ct > 0 {
                            let w = self.width; // get w before mutually borrowing
                            self.get_cell_mut(i*w + j).content = Content::Number(ct);
                        }
                    },
                    _ => {}
                }               
            }
        }
    }

    fn clear(&mut self) {
        for i in 0..self.width*self.height {
            self.get_cell_mut(i).clear();
        }
    }

    pub fn reveal(&mut self, i: u32) -> &Content {
        let cell = self.get_cell_mut(i);
        cell.reveal();
        &cell.content
    }

    fn get_cell_mut(&mut self, i:u32) -> &mut Cell {
        self.cells.get_mut(i as usize)
            .unwrap_or_else(|| panic!("Range check error at Field::get_cell_mut ({})", i))
    }

    fn get_cell(& self, i:u32) -> &Cell {
        self.cells.get(i as usize)
            .unwrap_or_else(|| panic!("Range check error at Field::get_cell ({})", i))
    }

    pub fn get_content(& self, i: u32) -> &Content {
        &self.get_cell(i).content
    }

    fn is_bomb_safe(&self, i: i32, j: i32) -> Option<u8> {
        if (i < 0) || ((i as u32) >= self.width) {
            None
        } else if (j < 0) || ((j as u32) >= self.height) {
            None
        } else {
            match self.get_cell((i as u32)*self.width + (j as u32)).content {
                Content::Bomb => Some(1),
                _  => Some(0)
            }
        }
    }
}