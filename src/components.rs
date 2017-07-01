extern crate specs;

use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Name(pub String);
#[derive(Debug)]
pub struct Player(pub i32);
#[derive(Debug)]
pub struct Position(pub u32, pub u32);
#[derive(Debug)]
pub struct Move(pub i32, pub i32);
#[derive(Debug)]
pub struct Blocking;
#[derive(Debug)]
pub struct FloorTrap {
    pub damage: i32,
}
#[derive(Debug)]
pub struct Sprite {
    pub ch: char,
    pub layer: String,
    pub alpha: u8,
    pub color: (u8, u8, u8),
    pub bkgnd: (u8, u8, u8),
}


impl Component for Name {
    type Storage = VecStorage<Name>;
}
impl Component for Player {
    type Storage = VecStorage<Player>;
}
impl Component for Position {
    type Storage = VecStorage<Position>;
}
impl Component for Move {
    type Storage = VecStorage<Move>;
}
impl Component for Blocking {
    type Storage = VecStorage<Blocking>;
}
impl Component for FloorTrap {
    type Storage = VecStorage<FloorTrap>;
}
impl Component for Sprite {
    type Storage = VecStorage<Sprite>;
}