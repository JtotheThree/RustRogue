extern crate rand;

use bear_lib_terminal::terminal;
use bear_lib_terminal::Color;

use rand::Rng;

use constants::*;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile{blocked: false, block_sight: false}
    }

    pub fn wall() -> Self {
        Tile{blocked: true, block_sight: true}
    }
}

pub struct Map {
    pub width: u32,
    pub height: u32,
    map: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        Map {
            width: width,
            height: height,
            map: vec![vec![Tile::empty();
                           height as usize];
                           width as usize],
        }
    }

    pub fn make_map(&mut self) {
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                let mut rng = rand::thread_rng();
                let chance: u32 = rng.gen_range(0, 10);
                if chance > 6 {
                    self.map[x][y] = Tile::wall();
                } else {
                    self.map[x][y] = Tile::empty();
                }
            }
        }
        //self.map[30][22] = Tile::wall();
        //self.map[50][22] = Tile::wall();
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Tile {
        self.map[x as usize][y as usize]
    }

    pub fn render(&self) {
        terminal::layer(0);

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let wall = self.map[x as usize][y as usize].block_sight;
                if wall {
                    terminal::set_background(Color::from_rgb(0, 100, 0));
                    terminal::set_foreground(Color::from_rgb(200, 80, 80));
                    terminal::put_xy(x as i32, y as i32, '#')
                } else {
                    terminal::set_background(Color::from_rgb(40, 0, 0));
                    terminal::set_foreground(Color::from_rgb(40, 40, 40));
                    terminal::put_xy(x as i32, y as i32, '.')
                }
            }
        }
    }
}