use field::{Field, Content};
use piston_window::*;

pub struct Game {
    field: Field,
    glyphs: Glyphs
}

impl Game {
    pub fn new(glyphs: Glyphs) -> Game {
        Game {
            field: Field::new(20, 20, 50),
            glyphs: glyphs
        }
    }

    pub fn render(&mut self, window: &PistonWindow) {      
        self.draw_field(window);      
    }

    fn draw_field(&mut self, window: &PistonWindow) {
        let w = window.size().width as f64;
        let h = window.size().height as f64;
        let cell_h = h / (self.field.height as f64);
        let cell_w = w / (self.field.width as f64);
        window.draw_2d(|c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            for i in 0..self.field.width {
                for j in 0..self.field.height {
                    match *self.field.get_content(i*self.field.width + j) {
                        Content::Bomb => {
                            rectangle([1.0, 0.0, 0.0, 1.0],
                                      [(i as f64)*cell_w, (j as f64)*cell_h, cell_w, cell_h],
                                      c.transform, g);

                        },
                        Content::Number(n) => {
                            let transform = c.transform.trans((i as f64)*cell_w + 10.0, (j as f64)*cell_h + cell_h - 5.0);
                            text::Text::colored([1.0, 1.0, 1.0, 1.0], 32).draw(
                                &*n.to_string(),
                                &mut self.glyphs,
                                &c.draw_state,
                                transform, g
                            );
                        },
                        Content::None => {
                            rectangle([1.0, 1.0, 1.0, 1.0],
                                      [(i as f64)*cell_w, (j as f64)*cell_h, cell_w, cell_h],
                                      c.transform, g);
                        }
                    }
                }
            }
        }); 
    }

    pub fn proc_key(&self, button: Button) {
        println!("{:?}", button);
    }
}