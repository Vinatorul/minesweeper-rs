
pub enum Content {
    Number(u8),
    Bomb,
    None
}

pub struct Cell {
    index: u32,
    content: Content,
    revialed: bool
}

impl Cell {
    pub fn clear(&mut self) {
        self.content = Content::None;
        self.revialed = false;
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
                           revialed: false});
        }
        field.fill();
        field
    }

    fn fill(&mut self) {
        self.clear();
        for i in 0..self.mines {

        }
    }

    fn clear(&mut self) {
        for i in 0..self.width*self.height {
            self.cells.get_mut(i as usize).unwrap().clear();
        }
    }
}