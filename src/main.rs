extern crate piston;

mod game;

use game::*;

use piston::event_loop::*;
use piston::input::*;

fn main() {
	let mut app = App::new();

	let player = app.game.allocate_entity();

	app.game.positions[player] = Some(Vector2D { x: 20.0, y: 100.0 });
	app.game.velocities[player] = Some(Vector2D { x: 1.0, y: 0.2 });

	let mut events = app.window.events();
	while let Some(e) = events.next(&mut app.window) {
		if let Some(r) = e.render_args() {
			app.render(&r);
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

		if let Some(u) = e.update_args() {
			app.update(&u);
		}

		// if let Some(k) = e.press_args() {
		// 	app.handle_keydown(&k);
		// }

		// if let Some(k) = e.release_args() {
		// 	app.handle_keyup(&k);
		// }
	}
}
