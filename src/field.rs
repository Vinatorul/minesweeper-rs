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
        for _i in 1..self.mines {
            let ind = rand::thread_rng().gen_range(0, self.width*self.height);
            self.get_cell_mut(ind).content = Content::Bomb
        }
        for i in 1..self.width*self.height {
            let cell = self.get_cell_mut(i)
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
}