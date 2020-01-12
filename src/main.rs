#[macro_use]
extern crate specs_derive;

use rltk::{Rltk, RGB};
use specs::prelude::*;

mod component;
mod entity;
mod map;
mod player;
mod rect;
mod state;
mod visibility;

use crate::component::Viewshed;
use crate::entity::{Player, Position, Renderable};
use crate::map::Map;
use crate::state::State;
use crate::visibility::VisibilitySystem;

fn main() {
    let context =
        Rltk::init_simple8x8(80, 50, "MORTAL WOMBAT", "resources");
    let mut gs = State {
        ecs: World::new(),
        systems: DispatcherBuilder::new()
            .with(VisibilitySystem {}, "visibility_system", &[])
            .build(),
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    let map = Map::new_room_corridors();
    gs.ecs.insert(map.clone());

    // start player in center of room 1
    let (p_x, p_y) = map.rooms[0].center();
    gs.ecs
        .create_entity()
        .with(Position { x: p_x, y: p_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: vec![],
            range: 8,
            dirty: true,
        })
        .build();

    map.rooms.iter().skip(1).for_each(|room| {
        let (x, y) = room.center();
        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph: rltk::to_cp437('g'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .build();
    });

    rltk::main_loop(context, gs);
}
