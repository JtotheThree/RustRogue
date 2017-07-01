use bear_lib_terminal::Color;
use bear_lib_terminal::terminal::{self};
use constants::*;
use components::{Sprite, Position};
use specs::{ReadStorage, System};

pub struct Render;

impl<'a> System<'a> for Render {
    type SystemData = (ReadStorage<'a, Position>,
                       ReadStorage<'a, Sprite>);

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (pos, spr) = data;

        for (pos, spr) in (&pos, &spr).join() {
            if pos.0 > SCREEN_WIDTH || pos.1 > SCREEN_HEIGHT {
                continue;
            }

            match spr.layer.as_ref() {
                "background"    => terminal::layer(0),
                "foreground"    => terminal::layer(1),
                "blocking"      => terminal::layer(2),
                _               => continue,
            }

            terminal::set_background(Color::from_rgb(spr.bkgnd.0, spr.bkgnd.1, spr.bkgnd.2));
            terminal::set_foreground(Color::from_rgb(spr.color.0, spr.color.1, spr.color.2));
            terminal::put_xy(pos.0 as i32, pos.1 as i32, spr.ch);
        }
    }
}
