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
use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone)]
pub struct Vector2D {
	pub x: f64,
	pub y: f64,
}

impl Vector2D {
	fn new() -> Vector2D {
		Vector2D { x: 0.0, y: 0.0 }
	}

	fn add(&self, other: Vector2D) -> Vector2D {
		Vector2D { x: self.x + other.x, y: self.y + other.y }
	}

	fn mul(&self, scale: f64) -> Vector2D {
		Vector2D { x: self.x * scale, y: self.y * scale }
	}

	fn to_unit(&self) -> Vector2D {
		let len = self.len();

		Vector2D { x: self.x / len, y: self.y / len }
	}

	fn len_sq(&self) -> f64 {
		self.x * self.x + self.y * self.y
	}

	fn len(&self) -> f64 {
		(self.x * self.x + self.y * self.y).sqrt()
	}
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Flag {
	Movement,
	Hidden,

	Input {
		jump: Key,

		left: Key,
		right: Key,
		up: Key,
		down: Key
	}
}

#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
	pub x: f64,
	pub y: f64,
	pub w: f64,
	pub h: f64
}

const PIXELS_PR_UNIT: f64 = 32.0 / 1.0;

impl Rectangle {
	fn new() -> Rectangle {
		Rectangle { x: 0.0, y: 0.0, w: 0.0, h: 0.0 }
	}

	pub fn intersects(&self, other: &Rectangle) -> bool {
		(self.x < other.x + other.w) &&
		(self.x + self.w  > other.x) &&
		(self.y < other.y + other.h) &&
		(self.y + self.h  > other.y)
	}

	fn min(&self) -> Vector2D {
		Vector2D { x: self.x, y: self.y }
	}

	fn max(&self) -> Vector2D {
		Vector2D { x: self.x + self.w, y: self.y + self.h }
	}

	fn translate(&self, x: f64, y: f64) -> Rectangle {
		Rectangle { x: self.x + x, y: self.y + y, w: self.w, h: self.h }
	}

	fn to_scaled_array(&self, scale: f64) -> [f64; 4] {
		[self.x * scale, self.y * scale, self.w * scale, self.h * scale]
	}

	fn to_array(&self) -> [f64; 4] {
		[self.x, self.y, self.w, self.h]
	}
}

#[derive(Debug)]
pub struct Entity {
	pub flags:    HashSet<Flag>,

	pub position: Vector2D, // u
	pub velocity: Vector2D, // u/s

	pub touch:    Vector2D, // intersections with walls/ground

	pub hitbox:   Rectangle
}

impl Entity {
	pub fn new() -> Entity {
		Entity {
			flags: HashSet::new(),

			position: Vector2D::new(),
			velocity: Vector2D::new(),

			touch: Vector2D::new(),

			hitbox: Rectangle::new(),
		}
	}

	pub fn get_hitbox(&self) -> Rectangle {
		self.hitbox.translate(self.position.x, self.position.y)
	}
}

pub const ENTITY_COUNT: usize = 32;

#[derive(Debug)]
pub struct Game {
	pub i: u32,

	pub entities: Vec<Entity>,
	pub walls: Vec<Rectangle>
}

impl Game {
	pub fn new() -> Game {
		Game {
			i: 0,
			entities: Vec::with_capacity(ENTITY_COUNT),
			walls: Vec::with_capacity(32),
		}
	}

	pub fn update(&mut self, keys: &HashSet<Key>, args: &UpdateArgs) {
		let gravity = Vector2D { x: 0.0, y: 9.82 }; // u/s^2

		let speed = 10.0; // u/s^2

		for entity in &mut self.entities {
			for flag in entity.flags.iter() {
				match flag {

					&Flag::Movement => {
						let drag_x = 1.0 - args.dt * 1.5;

						entity.velocity.x = entity.velocity.x * drag_x + gravity.x * args.dt;
						entity.velocity.y = entity.velocity.y + gravity.y * args.dt;

						let movement = entity.velocity.mul(args.dt / 4.0);

						entity.touch.x = 0.0;
						entity.touch.y = 0.0;

						// move in quaters
						'move_x: for i in 0..4 {
							entity.position.x += movement.x;

							let hitbox = entity.get_hitbox();

							for wall in &self.walls {
								if hitbox.intersects(wall) {
									// move back to undo intersection
									entity.position.x -= movement.x;

									entity.touch.x = movement.x.signum();

									entity.velocity.x = 0.0;
									break 'move_x;
								}
							}
						}

						'move_y: for i in 0..4 {
							entity.position.y += movement.y;

							let hitbox = entity.get_hitbox();

							for wall in &self.walls {
								if hitbox.intersects(wall) {
									// move back to undo intersection
									entity.position.y -= movement.y;

									entity.touch.y = movement.y.signum();

									entity.velocity.y = 0.0;
									break 'move_y;
								}
							}
						}
					},

					&Flag::Input {left, right, up, down, jump} => {
						if keys.contains(&left) {
							entity.velocity.x -= speed * args.dt;
						}
						if keys.contains(&right) {
							entity.velocity.x += speed * args.dt;
						}
						if keys.contains(&jump) && entity.touch.y == 1.0 {
							entity.velocity.y -= speed;
						}
					},

					_  => {}
				}
			}
		}
	}

	pub fn create_wall(&mut self, x: f64, y: f64, w: f64, h: f64) -> Rectangle {
		let wall = Rectangle { x: x, y: y, w: w, h: h };

		self.walls.push(wall);

		wall.clone()
	}

	pub fn spawn_entity<F>(&mut self, f: F)
		where F : Fn(&mut Entity)
	{
		let mut e = Entity::new();

		f(&mut e);

		self.entities.push(e);
	}
}

pub struct App {
	pub gl: GlGraphics,
	pub window: Window,

	pub keys: HashSet<Key>,
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

			keys: HashSet::new()
		}
	}

	pub fn handle_keydown(&mut self, key: Key) {
		self.keys.insert(key);
	}

	pub fn handle_keyup(&mut self, key: Key) {
		self.keys.remove(&key);
	}

	pub fn update(&mut self, args: &UpdateArgs, game: &mut Game) {
		game.update(&self.keys, args);
	}

	pub fn render(&mut self, args: &RenderArgs, game: &Game) {
		use self::graphics::*;

		const GREEN: [f32; 4] = [ 0.0, 1.0, 0.0, 1.0 ];
		const RED:   [f32; 4] = [ 1.0, 0.0, 0.0, 1.0 ];

		let mut square = rectangle::square(0.0, 1.0, 50.0);

		self.gl.draw(args.viewport(), |c, gl| {
			clear(GREEN, gl);

			let transform = c.transform.trans(0.0, 0.0);

			for entity in &game.entities {
				if !entity.flags.contains(&Flag::Hidden) {
					let ref pos = entity.position;

					// hitboxes are prescaled with 'PIXELS_PR_UNIT'
					rectangle(RED, entity.get_hitbox().to_scaled_array(PIXELS_PR_UNIT), transform, gl);
				}
			}


			for wall in &game.walls {
				rectangle(RED, wall.to_scaled_array(PIXELS_PR_UNIT), transform, gl);
			}
		});
	}
}
