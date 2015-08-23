extern crate piston_window;

use piston_window::*;

fn main() {
    let window: PistonWindow =
        WindowSettings::new("Minesweeper", [640, 480])
        .exit_on_esc(true).build().unwrap();
    for e in window {
        
    }
}