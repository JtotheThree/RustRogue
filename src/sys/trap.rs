use constants::*;
use components::{FloorTrap, Position, Sprite, Name};
use resources::CollisionMap;
use specs::{ReadStorage, WriteStorage, System, FetchMut, Entity, World};

pub struct Trap;

impl<'a> System<'a> for Trap {
    type SystemData = (
        FetchMut<'a, CollisionMap>,
        ReadStorage<'a, FloorTrap>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Sprite>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (colmap, trap, pos, name, mut sprite) = data;

        for (trap, pos, sprite) in (&trap, &pos, &mut sprite).join() {
            if colmap.contains_key(&(pos.0, pos.1)) {
                if sprite.layer == "trap".to_string() {
                    sprite.layer = "foreground".to_string();
                    let target: &Entity = colmap.get(&(pos.0, pos.1)).unwrap();
                    println!("Target Ent: {:?}", target);
                    if let Some(target_name) = name.get(*target) {
                        println!("{} stepped on a trap.", target_name.0);
                    }

                    /*match target_name {
                        Some(target_name) => {
                            println!("The idiot {} stepped on a trap.", target_name.0)
                        },
                        None => println!("Something unknown triggered the trap."),
                    }*/
                }
            }
        }
    }
}