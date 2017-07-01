use components::{Blocking, Move, Position};
use resources::CollisionMap;
use specs::{ReadStorage, WriteStorage, System, FetchMut, Entities};

pub struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (
        Entities<'a>,
        FetchMut<'a, CollisionMap>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Move>,
        ReadStorage<'a, Blocking>
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (ent, mut colmap, mut pos, mov, _) = data;

        for (ent, pos, mov) in (&*ent, &mut pos, &mov).join() {
            let new_x = add_sub_i32!(pos.0, mov.0);
            let new_y = add_sub_i32!(pos.1, mov.1);

            if colmap.contains_key(&(new_x, new_y)) {
                continue;
            }

            colmap.remove(&(pos.0, pos.1));
            colmap.insert((new_x, new_y), ent);

            pos.0 = new_x;
            pos.1 = new_y;         
        }
    }
}