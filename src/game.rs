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
            field: Field::new(20, 20, 50),
            glyphs: glyphs,
            mouse_x: 0.0,
            mouse_y: 0.0
        }
    }

    pub fn render(&mut self, window: &PistonWindow) {      
        self.draw_field(window);      
    }

    fn draw_field(&mut self, window: &PistonWindow) {
        let w = window.size().width as f64;
        let h = window.size().height as f64;
        let cell_h = h / (self.field.get_height() as f64);
        let cell_w = w / (self.field.get_width() as f64);
        window.draw_2d(|c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            for i in 0..self.field.get_width() {
                for j in 0..self.field.get_height() {
                    if !self.field.revealed(i + j*self.field.get_height()) {
                        continue;
                    }
                    match *self.field.get_content(i + j*self.field.get_height()) {
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
                        let w = window.size().width;
                        let h = window.size().height;
                        let cell_h = h / self.field.get_height();
                        let cell_w = w / self.field.get_width();
                        let x = (self.mouse_x.floor() as u32)/cell_w; 
                        let y = (self.mouse_y.floor() as u32)/cell_h;
                        let h = self.field.get_height();
                        self.open_cell(x + y*h);
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
            _ => println!("ok {}", i)
        }
    }

    pub fn mouse_move(&mut self, mouse_rel: [f64; 2]) {
        self.mouse_x = mouse_rel[0];
        self.mouse_y = mouse_rel[1];
    }
}