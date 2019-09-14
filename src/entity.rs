use rltk::RGB;
use specs::prelude::*;

#[derive(Debug, Component)]
pub struct Player {}

#[derive(Debug, Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Component)]
pub struct LeftMover {}

pub struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    type SystemData =
        (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= -1;
            pos.y -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
            if pos.x > 79 {
                pos.x = 0;
            }
            if pos.y < 0 {
                pos.y = 49;
            }
            if pos.y > 49 {
                pos.y = 0;
            }
        }
    }
}
