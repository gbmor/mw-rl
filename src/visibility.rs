use super::{Map, Position, Viewshed};
use rltk::{field_of_view, Point};
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, mut viewshed, pos) = data;
        (&mut viewshed, &pos).join().into_iter().for_each(
            |(viewshed, pos)| {
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles = field_of_view(
                    Point::new(pos.x, pos.y),
                    viewshed.range,
                    &*map,
                );
            },
        );
    }
}
