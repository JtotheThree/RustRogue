use components::{Position, Move};
use resources::PositionMap;
use specs::{ReadStorage, WriteStorage, System, FetchMut, RunNow, Entities};

pub struct Positions;

impl<'a> System<'a> for Positions {
    type SystemData = (
            Entities<'a>,
            FetchMut<'a, PositionMap>,
            ReadStorage<'a, Position>,
            ReadStorage<'a, Move>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (ent, mut posmap, pos, mov) = data;
        
        posmap.clear();

        for (ent, pos) in (&*ent, &pos).join() {
            posmap.insert((pos.0, pos.1), ent);
        }
    }
}