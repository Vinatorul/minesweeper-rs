use field::{Field, Content};
use piston_window::*;

pub struct Game {
    field: Field,
    glyphs: Glyphs,
    mouse_x: f64,
    mouse_y: f64
}

impl Game {
    pub fn new(glyphs: Glyphs) -> Game {
        Game {
            field: Field::new(20, 15, 50),
            glyphs: glyphs,
            mouse_x: 0.0,
            mouse_y: 0.0
        }
    }

    pub fn render(&mut self, window: &PistonWindow) {      
        window.draw_2d(|c, g| {
            let max_field_size = self.get_max_field_size(window);
            self.draw_field(c, g, max_field_size);
        });
    }

    fn get_max_field_size(&self, window: &PistonWindow) -> [u32; 2] {
        [2*window.size().width/3, window.size().height]
    }

    fn draw_field(&mut self, context: Context, graphics: &mut G2d, max_field_size: [u32; 2]) {
        let cell_w = (max_field_size[0] / self.field.get_width()) as f64;
        let cell_h = (max_field_size[1] / self.field.get_height()) as f64;
        clear([0.0, 0.0, 0.0, 1.0], graphics);
        for i in 0..self.field.get_width() {
            for j in 0..self.field.get_height() {
                if !self.field.revealed(i + j*self.field.get_width()) {
                    continue;
                }
                match *self.field.get_content(i + j*self.field.get_width()) {
                    Content::Bomb => {
                        rectangle([1.0, 0.0, 0.0, 1.0],
                                  [(i as f64)*cell_w, (j as f64)*cell_h, cell_w, cell_h],
                                  context.transform,
                                  graphics);

                    },
                    Content::Number(n) => {
                        let transform = context.transform.trans((i as f64)*cell_w + 10.0, (j as f64)*cell_h + cell_h - 5.0);
                        text::Text::colored([1.0, 1.0, 1.0, 1.0], 32).draw(
                            &*n.to_string(),
                            &mut self.glyphs,
                            &context.draw_state,
                            transform,
                            graphics
                        );
                    },
                    Content::None => {
                        rectangle([1.0, 1.0, 1.0, 1.0],
                                  [(i as f64)*cell_w, (j as f64)*cell_h, cell_w, cell_h],
                                  context.transform,
                                  graphics);
                    }
                }
            }
        }
        let field_size = [cell_w*(self.field.get_width() as f64), cell_h*(self.field.get_height() as f64)];
        for i in 0..self.field.get_width()+1 {
            line::Line::new([0.5, 0.5, 0.5, 1.0], 1.0)
                .draw([(i as f64)*cell_w, 0.0, (i as f64)*cell_w, field_size[1]],
                      &context.draw_state,
                      context.transform,
                      graphics);
            line::Line::new([0.5, 0.5, 0.5, 1.0], 1.0)
                .draw([0.0, (i as f64)*cell_h, field_size[0], (i as f64)*cell_h],
                      &context.draw_state,
                      context.transform,
                      graphics);
        }
    }

    pub fn proc_key(&mut self, button: Button, window: &PistonWindow) {
        match button {
            Button::Keyboard(key) => {
                match key {
                    Key::R => self.field.restart(),
                    _ => println!("{:?}", key)
                }
            },
            Button::Mouse(btn) => {
                match btn {
                    MouseButton::Left => {
                        let max_field_size = self.get_max_field_size(window);
                        let cell_w = max_field_size[0] / self.field.get_width();
                        let cell_h = max_field_size[1] / self.field.get_height();
                        if (self.mouse_x > (cell_w*self.field.get_width()) as f64) ||
                            (self.mouse_y > (cell_h*self.field.get_height())  as f64) {
                                return;
                            }
                        let x = (self.mouse_x.floor() as u32)/cell_w; 
                        let y = (self.mouse_y.floor() as u32)/cell_h;
                        let w = self.field.get_width();
                        self.open_cell(x + y*w);
                    },
                    _ => println!("{:?}", btn)
                }
            }
        }
    }

    fn open_cell(&mut self, i: u32) {
        match *self.field.reveal(i) {
            Content::Bomb => {
                self.field.reveal_all();
                println!("Game over =(, {}", i);
            },
            Content::None => {
               self.field.chain_reveal(i);
            }
            _ => println!("ok {}", i)
        }
    }

    pub fn mouse_move(&mut self, mouse_rel: [f64; 2]) {
        self.mouse_x = mouse_rel[0];
        self.mouse_y = mouse_rel[1];
    }
}