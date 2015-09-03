extern crate piston_window;
#[macro_use]
extern crate clap;
extern crate rand;
extern crate find_folder;

mod game;
mod field;
mod ui;
mod common;

use piston_window::*;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Minesweeper")
        .author("Alexander Kuvaev <alexander@kuvaev.me>")
        .version(&*format!("v{}", crate_version!()))
        .about("Simple minesweeper in Rust ")
        .arg(Arg::from_usage("-s, --size [size] 'window size \"width\"x\"height\"'")
            .validator(|val| {
                let mut s = val.split("x");
                match s.next() {
                    Some(w) => {
                        match w.parse::<u32>() {
                            Ok(r) => {
                                if (r < 100) || (r > 4096) {
                                    return Err(format!("window width must be between 100 and 4096"))
                                }
                            },
                            Err(e) => return Err(format!("window width arg error: {}", e))
                        }
                    }
                    None => return Err(format!("window size expected \"width\"x\"height\", got {}", val))
                }
                match s.next() {
                    Some(h) => {
                        match h.parse::<u32>() {
                            Ok(r) => {
                                if (r < 100) || (r > 3072) {
                                    return Err(format!("window height must be between 100 and 3072"))
                                }
                            },
                            Err(e) => return Err(format!("window height arg error: {}", e))
                        }
                    }
                    None => return Err(format!("window size expected \"width\"x\"height\", got {}", val))
                }
                match s.next() {
                    Some(_r) => Err(format!("window size arg expected \"width\"x\"height\", got {}", val)),
                    None => Ok(())
                }
            }))
        .arg(Arg::from_usage("-f, --field [field] 'field size \"width\"x\"height\"'")
            .validator(|val| {
                let mut s = val.split("x");
                match s.next() {
                    Some(w) => {
                        match w.parse::<u32>() {
                            Ok(r) => {
                                if (r < 5) || (r > 100) {
                                    return Err(format!("field width must be between 5 and 100"))
                                }
                            },
                            Err(e) => return Err(format!("field width arg error: {}", e))
                        }
                    }
                    None => return Err(format!("field size expected \"width\"x\"height\", got {}", val))
                }
                match s.next() {
                    Some(h) => {
                        match h.parse::<u32>() {
                            Ok(r) => {
                                if (r < 5) || (r > 100) {
                                    return Err(format!("field height must be between 5 and 100"))
                                }
                            },
                            Err(e) => return Err(format!("field height arg error: {}", e))
                        }
                    }
                    None => return Err(format!("field size expected \"width\"x\"height\", got {}", val))
                }
                match s.next() {
                    Some(_r) => Err(format!("field size arg expected \"width\"x\"height\", got {}", val)),
                    None => Ok(())
                }
            }))
        .arg(Arg::from_usage("-m, --mines [mines] 'max mines'"))
        .arg(Arg::from_usage("--oldOGL 'set OpenGL version to 2.1'"))
        .get_matches();

    let mut width = 1024;
    let mut height = 768;
    let mut f_width = 20;
    let mut f_height = 20;
    let mut mines = 50;
    let mut opengl = OpenGL::V3_2;
    if let Some(size) = matches.value_of("size") {
        let mut s = size.split("x");
        width = s.next().unwrap().parse().unwrap_or(width);
        height = s.next().unwrap().parse().unwrap_or(height);
    }
    if let Some(field) = matches.value_of("field") {
        let mut s = field.split("x");
        f_width = s.next().unwrap().parse().unwrap_or(f_width);
        f_height = s.next().unwrap().parse().unwrap_or(f_height);
    }
    if let Some(m) = matches.value_of("mines") {
        mines = m.parse().unwrap_or(mines);
    }
    if letmatches.is_present("oldOGL") {
        opengl = OpenGL::V2_1;
    }

    let window: PistonWindow =
        WindowSettings::new("Minesweeper", [width, height])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.borrow().clone();
    let glyphs = Glyphs::new(font, factory).unwrap();

    let mut game = game::Game::new(glyphs, f_width, f_height, mines);

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
