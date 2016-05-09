extern crate piston;

mod game;

use game::*;

use piston::event_loop::*;
use piston::input::*;

fn main() {
	let mut app = App::new();
	let mut game = Game::new();

	game.spawn_entity(|ref mut e| {
		use game::Flag::*;
		use piston::input::Key::*;

		e.flags.insert(Movement);
		e.flags.insert(Input {
			jump: Space,

			left: Left,
			right: Right,
			up: Up,
			down: Down
		});

		e.velocity.x = 1.0;

		e.hitbox.w = 0.4;
		e.hitbox.h = 1.2;
	});

	/*
	game.spawn_entity(|ref mut e| {
		use game::Flag::*;
		use piston::input::Key::*;

		e.flags.insert(Movement);
		e.flags.insert(Input {
			jump: Space,

			left: A,
			right: D,
			up: W,
			down: S
		});

		e.velocity.x = 1.0;

		e.hitbox.w = 0.5;
		e.hitbox.h = 1.8;
	});
	*/

	let a = game.create_wall(2.0, 9.0, 0.5, 1.1);
	let b = game.create_wall(2.0, 10.0, 10.0, 1.0);

	println!("{:?}", a.intersects(&b));

	let mut events = app.window.events();
	while let Some(e) = events.next(&mut app.window) {
		if let Some(u) = e.update_args() {
			game.i += 1;
			app.update(&u, &mut game);
		}

		if let Some(r) = e.render_args() {
			app.render(&r, &game);
 		}

 		if let Some(k) = e.press_args() {
 			match k {
 				Button::Mouse(MouseButton::Left) => println!("Down"),
 				Button::Keyboard(w) => app.handle_keydown(w),
 				_ => {}
 			}
 		}

 		if let Some(k) = e.release_args() {
 			match k {
 				Button::Mouse(MouseButton::Left) => println!("Down"),
 				Button::Keyboard(w) => app.handle_keyup(w),
 				_ => {}
 			}
 		}
	}
}
