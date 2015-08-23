use field;
use piston_window::*;

pub struct Game {
    field: field::Field
}

impl Game {
    pub fn new() -> Game {
        Game {
            field: field::Field::new(20, 20, 30)
        }
    }

    pub fn render(&self, window: &PistonWindow) {
        window.draw_2d(|_c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
        });
    }

    pub fn proc_key(&self, button: Button) {
        println!("Pressed {:?}", button);
    }
}