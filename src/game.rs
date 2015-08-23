use field::{Field, Content};
use piston_window::*;

pub struct Game {
    field: Field
}

impl Game {
    pub fn new() -> Game {
        Game {
            field: Field::new(20, 20, 30)
        }
    }

    pub fn render(&self, window: &PistonWindow) {
        window.draw_2d(|c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
        });
        self.draw_field(window);
    }

    fn draw_field(&self, window: &PistonWindow) {
        let w = window.size().width as f64;
        let h = window.size().height as f64;
        let cell_h = h / (self.field.height as f64);
        let cell_w = w / (self.field.width as f64);
        for i in 0..self.field.width {
            for j in 0..self.field.height {
                match *self.field.get_content(i*self.field.width + j) {
                    Content::Bomb => {
                        window.draw_2d(|c, g| {
                            rectangle([1.0, 0.0, 0.0, 1.0],
                                      [(i as f64)*cell_w, (j as f64)*cell_h, cell_w, cell_h],
                                      c.transform, g);

                        });
                    },
                    Content::Number(n) => {

                    },
                    Content::None => {
                        window.draw_2d(|c, g| {
                            rectangle([1.0, 1.0, 1.0, 1.0],
                                      [(i as f64)*cell_w, (j as f64)*cell_h, cell_w, cell_h],
                                      c.transform, g);

                        });
                    }
                }
            }
        }
    }

    pub fn proc_key(&self, button: Button) {
        println!("{:?}", button);
    }
}