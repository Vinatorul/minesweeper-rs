extern crate piston_window;
extern crate clap;

use piston_window::*;
use clap::{Arg, App, AppSettings};

fn main() {
	let matches = App::new("Minesweeper")
		.author("Alexander Kuvaev <alexander@kuvaev.me>") 
		.version("0.1")
		.about("Simple minesweeper in Rust ")
		.arg(Arg::from_usage("--width [width] 'window width'"))
		.arg(Arg::from_usage("--height [height] 'window height'"))
		.get_matches();

	let mut width = 640;
	let mut height = 480;
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

    for e in window {
        e.draw_2d(|_c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
        });

        if let Some(button) = e.press_args() {
            println!("Pressed {:?}", button);
        }
    }
}