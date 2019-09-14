#[macro_use]
extern crate specs_derive;

use rltk::{Rltk, RGB};
use specs::prelude::*;

mod entity;
mod map;
mod player;
mod rect;
mod state;

use crate::entity::{Player, Position, Renderable};
use crate::state::State;

fn main() {
    let context =
        Rltk::init_simple8x8(80, 50, "MORTAL WOMBAT", "resources");
    let mut gs = State {
        ecs: World::new(),
        systems: DispatcherBuilder::new().build(),
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    let (rooms, map) = map::new_room_corridors();
    gs.ecs.insert(map);

    // start player in center of room 1
    let (p_x, p_y) = rooms[0].center();
    gs.ecs
        .create_entity()
        .with(Position { x: p_x, y: p_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    rltk::main_loop(context, gs);
}
