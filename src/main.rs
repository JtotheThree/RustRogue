#![feature(proc_macro)]
extern crate bear_lib_terminal;
extern crate cgmath;
extern crate specs;
extern crate rand;

use bear_lib_terminal::terminal::{self, config};
use specs::{System, RunNow, World, ReadStorage};

#[macro_use]
mod macros;

mod components;
mod constants;
mod fps;
mod gen;
mod map;
mod resources;
mod sys;

use map::{Map};
use components::{Player, Name, Position, Move, Sprite, Blocking, FloorTrap};
use resources::{Exit, CollisionMap, PositionMap};
use constants::*;


fn main() {
	use specs::DispatcherBuilder;

	terminal::open("Simple example", SCREEN_WIDTH, SCREEN_HEIGHT);
	terminal::set(config::Window::empty().resizeable(true));
	terminal::set(config::Output::clean().vsync(false));
	terminal::set(config::ConfigPart::to_config_str(&"font: res/Yun_16x16_sm.png, size=16x16, codepage=437"));
	terminal::refresh();

	let mut world = World::new();
	world.register::<Player>();
	world.register::<Position>();
	world.register::<Name>();
	world.register::<Move>();
	world.register::<Sprite>();
	world.register::<Blocking>();
	world.register::<FloorTrap>();

	world.add_resource(Exit(false));
	world.add_resource(CollisionMap::new());
	world.add_resource(PositionMap::new());

	world.create_entity().with(Player(0))
						 .with(Position(32, 20))
						 .with(Move(0, 0))
						 .with(Blocking)
						 .with(Sprite{ch: '@', layer: "blocking".to_string(), alpha: 255,
						 	   color: (255, 255, 255), bkgnd: (0, 0, 0)})
						 .with(Name("Player".to_string()))
						 .build();

	world.create_entity().with(Position(32,21))
						 .with(Sprite{ch: 'x', layer: "trap".to_string(), alpha: 255,
						 	   color: (200, 40, 40), bkgnd: (0,0,0)})
						 .with(FloorTrap{damage: 10})
						 .build();

	let mut dispatcher = DispatcherBuilder::new()
				.add_thread_local(sys::input::Input)
				.add(sys::movement::Movement, "Movement", &[])
				// .add(gen::positions::Positions, "Positions", &["Movement"])
				.add(sys::render::Render, "Render", &[])
				.add(sys::trap::Trap, "Trap", &[])
				.build();

	let mut map = Map::new(MAP_WIDTH, MAP_HEIGHT);
	map.make_map();

	if !REND_MAP {
		for y in 0..map.height {
			for x in 0..map.width {
				let tile = map.get_tile(x, y);
				let color: (u8, u8, u8);
				let bkgnd: (u8, u8, u8);

				if tile.blocked {
					let tile_ch = '#';
					color = (200, 20, 200);
					bkgnd = (60, 60, 60);
					world.create_entity().with(Position(x, y))
										 .with(Blocking)
									     .with(Sprite{ch: tile_ch, layer: "blocking".to_string(), alpha: 255,
									 		          color: color, bkgnd: bkgnd})
									     .build();
				} else {
					let tile_ch = '.';
					color = (40, 40, 40);
					bkgnd = (30, 30, 30);
					world.create_entity().with(Position(x, y))
									     .with(Sprite{ch: tile_ch, layer: "background".to_string(), alpha: 255,
									 		          color: color, bkgnd: bkgnd})
									     .build();
				}
			}
		}
	}

	let mut generate_collision = gen::collision::Collision;
	generate_collision.run_now(&world.res);

	let mut fps = fps::Counter::new();
	let mut avg_fps = 0;

	loop {
		terminal::clear(None);

		if REND_MAP {
			map.render();
		}

		dispatcher.dispatch(&mut world.res);

		terminal::refresh();

		let exit = world.read_resource::<Exit>();
		let exit = exit.0;
		if exit {
			break;
		}

		avg_fps = fps.tick();
		// Print FPS to Console every frame
		print!("\rFPS: {}", avg_fps);
	}

	//Dump FPS
	println!("\nAverage FPS: {}", avg_fps);

	terminal::close();
}