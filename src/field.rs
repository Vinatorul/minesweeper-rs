use rand;
use rand::Rng;

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
                match self.get_cell(i + j*self.height).content {
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
                            let h = self.height; // get h before mutually borrowing
                            self.get_cell_mut(i + j*h).content = Content::Number(ct);
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

    pub fn get_content(& self, i: u32) -> &Content {
        &self.get_cell(i).content
    }

    fn is_bomb_safe(&self, i: i32, j: i32) -> Option<u8> {
        if (i < 0) || ((i as u32) >= self.width) {
            None
        } else if (j < 0) || ((j as u32) >= self.height) {
            None
        } else {
            match self.get_cell((i as u32)+ (j as u32)*self.height).content {
                Content::Bomb => Some(1),
                _  => Some(0)
            }
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
      for i in 0..self.width*self.height {
            self.get_cell_mut(i).revealed = true;
        }  
    }
}