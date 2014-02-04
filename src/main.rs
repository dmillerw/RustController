extern mod native;
extern mod rsfml;

use rsfml::window::{ContextSettings, VideoMode, event, Close};
use rsfml::graphics::{RenderWindow, CircleShape, Color};

use std::io;

use controller::{ControllerStack, ControllerMessage, TraitController};

// I still don't understand why these go BELOW use statements
pub mod controller;

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
	let args = std::os::args();

	let mut x_size: uint = 800;
	let mut y_size: uint = 600;

	let mut fail = false;

	if args.len() == 3 {
		// Retrieved and formed into Option values, which we
		// then check in the match block below
		let arg_1 = from_str::<uint>(args[1]);
		let arg_2 = from_str::<uint>(args[2]);

		// Match statements are wonderful and awesome :D
		match arg_1 {
			Some(x) => x_size = x,
			None      => fail = true,
		}

		match arg_2 {
			Some(y) => y_size = y,
			None      => fail = true,
		}
	} else if fail || args.len() != 1 {
		println!("Valid arguments are: { } [resolution_x] [resolution_y]", args[0]);
		return; // HALT
	}

	initialize_sfml(x_size, y_size, ~"Rust SFML Test");
}

fn initialize_sfml(x_size: uint, y_size: uint, title: ~str) {
    let mut window = match RenderWindow::new(VideoMode::new_init(x_size, y_size, 32), title, Close, &ContextSettings::default()) {
        Some(window) => window,
        None => fail!("Cannot create a new Render Window.")
    };

    let mut stack: ControllerStack = ControllerStack{controllers: ~[]};
    stack.push(~ControllerMessage{message: ~"Hello"});
    stack.peek().game_loop();

    while window.is_open() {
        loop {
            match window.poll_event() {
                event::Closed => window.close(),
                event::NoEvent => break,
                _ => {}
            }
        }

        window.clear(&Color::new_RGB(0, 200, 200));
        window.display()
    }
}