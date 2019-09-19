use super::{Map, Player, Position, Viewshed};
use rltk::{field_of_view, Point};
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;
        (&entities, &mut viewshed, &pos)
            .join()
            .into_iter()
            .for_each(|(ent, viewshed, pos)| {
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles = field_of_view(
                    Point::new(pos.x, pos.y),
                    viewshed.range,
                    &*map,
                );
                let p: Option<&Player> = player.get(ent);
                if let Some(p) = p {
                    viewshed.visible_tiles.iter().for_each(|vis| {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.revealed_tiles[idx] = true;
                    });
                }
            });
    }
}
