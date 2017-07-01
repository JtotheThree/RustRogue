use std::collections::HashMap;
use specs;

pub struct Exit(pub bool);
pub type CollisionMap = HashMap<(u32, u32), specs::Entity>;
pub type PositionMap = HashMap<(u32, u32), specs::Entity>;