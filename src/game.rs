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
            clear([0.0, 0.0, 0.0, 1.0], g);
            let field_rect = self.get_field_rect(window);
            self.field.draw(c, g, field_rect, &mut self.glyphs);
        });
    }

    fn get_field_rect(&self, window: &PistonWindow) -> [u32; 4] {
        let mut w = 2*window.size().width/3;
        w = (w /self.field.get_width()) * self.field.get_width();
        let mut h = window.size().height;
        h = (h /self.field.get_height()) * self.field.get_height();
        [0, 0, w, h]
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
                        let field_rect = self.get_field_rect(window);
                        let cell_w = field_rect[2] / self.field.get_width();
                        let cell_h = field_rect[3] / self.field.get_height();
                        let mouse_x = self.mouse_x.floor() as u32;
                        let mouse_y = self.mouse_y.floor() as u32;
                        if (mouse_x < field_rect[0]) || (mouse_x > field_rect[0] + field_rect[2]) ||
                           (mouse_y < field_rect[1]) || (mouse_y > field_rect[1] + field_rect[3]) {
                            return;
                        }
                        let x = (mouse_x - field_rect[0]) / cell_w;
                        let y = (mouse_y - field_rect[1]) / cell_h;
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