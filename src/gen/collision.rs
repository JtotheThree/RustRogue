use components::{Blocking, Move, Position};
use resources::CollisionMap;
use specs::{ReadStorage, WriteStorage, System, FetchMut, RunNow, Entities};

pub struct Collision;

impl<'a> System<'a> for Collision {
    type SystemData = (
            Entities<'a>,
            FetchMut<'a, CollisionMap>,
            ReadStorage<'a, Position>,
            ReadStorage<'a, Blocking>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        println!("Updating collision map.");

        let (ent, mut colmap, pos, block) = data;
        
        colmap.clear();

        for (ent, pos, _) in (&*ent, &pos, &block).join() {
            colmap.insert((pos.0, pos.1), ent);
        }
    }
}