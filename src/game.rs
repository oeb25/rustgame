extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use self::piston::window::WindowSettings;
use self::piston::event_loop::*;
use self::piston::input::*;
use self::glutin_window::GlutinWindow as Window;
use self::opengl_graphics::{ GlGraphics, OpenGL };

use std::ops::Add;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct Vector2D {
	pub x: f64,
	pub y: f64,
}

impl Vector2D {
	fn add(&self, other: Vector2D) -> Vector2D {
		Vector2D { x: self.x + other.x, y: self.y + other.y }
	}
}

// impl Add for Vector2D {
// 	type Output = Vector2D;

// 	fn add(self, other: Vector2D) -> Vector2D {
// 		Vector2D { x: self.x + other.x, y: self.y + other.y }
// 	}
// }

#[derive(Debug)]
pub struct Entity {
	position: Vector2D,
	velocity: Vector2D,
}

impl Entity {
	pub fn new() -> Entity {
		Entity { position: Vector2D { x: 0.0, y: 0.0 }, velocity: Vector2D { x: 0.0, y: 0.0 } }
	}
}

pub const ENTITY_COUNT: usize = 32;

#[derive(Debug, Copy, Clone)]
pub struct Game {
  pub masks: [Option<i32>; ENTITY_COUNT],

  pub positions: [Option<Vector2D>; ENTITY_COUNT],
  pub velocities: [Option<Vector2D>; ENTITY_COUNT],
}

impl Game {
	pub fn new() -> Game {
		Game {
			/* Måske unødvendig */
			masks: [None; ENTITY_COUNT],

			positions: [None; ENTITY_COUNT],
			velocities: [None; ENTITY_COUNT]
		}
	}

	pub fn update(&mut self, keys: &HashMap<Key, bool>, args: &UpdateArgs) {
		let gravity = Vector2D { x: 0.0, y: 0.05 };

		if keys.contains_key(&Key::Right) {
			match self.velocities[0] {
				Some(ref mut v) => {
					v.x += 1.0;
				},
				None => {}
			}
		}

		for i in 0..ENTITY_COUNT {
			// if self.positions[i].and(self.velocities[i]).is_some() {

			/*
			if self.positions[i].is_some() && self.velocities[i].is_some() {
				let mut pos = self.positions[i].as_mut().unwrap();
				let mut vel = self.velocities[i].as_mut().unwrap();

				vel.x = vel.x * 0.9 + gravity.x;
				vel.y = vel.y * 0.9 + gravity.y;

				pos.x += vel.x;
				pos.y += vel.y;
			}
			*/
			let mut movement = (self.positions[i], self.velocities[i]);

			match movement {
				(Some(ref mut pos), Some(ref mut vel)) => {
					vel.x = vel.x * 0.9 + gravity.x;
					vel.y = vel.y * 0.9 + gravity.y;

					pos.x += vel.x;
					pos.y += vel.y;

					println!("Pos: {:?}\nVec: {:?}", pos, vel);
				},
				_ => {}
			}

			self.positions[i] = movement.0;
			self.velocities[i] = movement.1;
		}
	}

	pub fn find_free_entity(&self) -> usize {
		let mut id: usize = ENTITY_COUNT;

		'run: for i in 0..ENTITY_COUNT {
			match self.masks[i] {
				Some(_) => {}
				None    => {
					id = i;
					break 'run;
				}
			}
		}

		id
	}

	pub fn allocate_entity(&mut self) -> usize {
		let id = self.find_free_entity();

		self.masks[id] = Some(0);

		id
	}

	pub fn deallocate(&mut self, id: usize) {
		self.masks[id].take();
		self.positions[id].take();
		self.velocities[id].take();
	}
}

pub struct App {
	pub gl: GlGraphics,
	pub window: Window,

	pub keys: HashMap<Key, bool>,
	pub game: Game,
}

impl App {
	pub fn new() -> App {
		let opengl = OpenGL::V3_2;

		let window = WindowSettings::new("Game", [600, 600])
			.opengl(opengl)
			.exit_on_esc(true)
			.build()
			.unwrap();

		App {
			gl: GlGraphics::new(opengl),
			window: window,

			keys: HashMap::new(),

			game: Game::new()
		}
	}

	pub fn handle_keydown(&mut self, key: Key) {
		self.keys.insert(key, true);
	}

	pub fn handle_keyup(&mut self, key: Key) {
		self.keys.remove(&key);
	}

	pub fn update(&mut self, args: &UpdateArgs) {
		// ?
		self.game.update(&self.keys, args);
	}

	pub fn render(&mut self, args: &RenderArgs) {
		use self::graphics::*;

		const GREEN: [f32; 4] = [ 0.0, 1.0, 0.0, 1.0 ];
		const RED:   [f32; 4] = [ 1.0, 0.0, 0.0, 1.0 ];

		let mut square = rectangle::square(0.0, 0.0, 50.0);

		// Copy eller ref?
		let game = &self.game;

		self.gl.draw(args.viewport(), |c, gl| {
			clear(GREEN, gl);

			for i in 0..ENTITY_COUNT {
				if game.positions[i].is_some() {
					let pos = game.positions[i].unwrap();
					println!("{:?}", pos);

					let transform = c.transform.trans(pos.x, pos.y);

					rectangle(RED, square, transform, gl);
				}
			}
		});
	}
}
