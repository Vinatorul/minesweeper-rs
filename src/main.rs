extern crate piston_window;
extern crate clap;
extern crate rand;
extern crate find_folder;

mod game;
mod field;

use piston_window::*;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Minesweeper")
        .author("Alexander Kuvaev <alexander@kuvaev.me>")
        .version("0.1")
        .about("Simple minesweeper in Rust ")
        .arg(Arg::from_usage("--width [width] 'window width'"))
        .arg(Arg::from_usage("--height [height] 'window height'"))
        .get_matches();

    let mut width = 1024;
    let mut height = 600;
    if let Some(w) = matches.value_of("width") {
        width = w.parse().unwrap_or(width);
    }
    if let Some(h) = matches.value_of("height") {
        height = h.parse().unwrap_or(height);
    }

    let window: PistonWindow =
        WindowSettings::new("Minesweeper", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.borrow().clone();
    let glyphs = Glyphs::new(font, factory).unwrap();

    let mut game = game::Game::new(glyphs);

    for e in window {
        game.render(&e);

        if let Some(mouse_rel) = e.mouse_cursor_args() {
            game.mouse_move(mouse_rel);
        }

        if let Some(button) = e.press_args() {
            game.proc_key(button, &e);
        }
    }
}