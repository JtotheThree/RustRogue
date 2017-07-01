use bear_lib_terminal::terminal::{self, Event, KeyCode};
use specs::{ReadStorage, WriteStorage, System, FetchMut};
use components::{Player, Move};
use resources::{Exit};

pub struct Input;

impl<'a> System<'a> for Input {
    type SystemData = (FetchMut<'a, Exit>,
                       WriteStorage<'a, Move>,
                       WriteStorage<'a, Player>);

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (mut exit, mut mov, mut player) = data;

        for (mov, _) in (&mut mov, &mut player).join() {
                mov.0 = 0;
                mov.1 = 0;

            	if !terminal::has_input() {
                    return
                }

                let event = terminal::read_event().unwrap();

                match event {
                    Event::Close | Event::KeyPressed{key: KeyCode::Escape, ..} => *exit = Exit(true),
                    Event::KeyPressed{key: KeyCode::Left, ..}				   => mov.0 = -1,
                    Event::KeyPressed{key: KeyCode::Right, ..}				   => mov.0 = 1,
                    Event::KeyPressed{key: KeyCode::Up, ..}				       => mov.1 = -1,
                    Event::KeyPressed{key: KeyCode::Down, ..}				   => mov.1 = 1,
                    _														   => (),
                }
        }
    }
}