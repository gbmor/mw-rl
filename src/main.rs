#[macro_use]
extern crate specs_derive;

use rltk::{Rltk, RGB};
use specs::prelude::*;

mod entity;
mod map;
mod player;
mod state;

use crate::entity::{
    LeftMover, LeftWalker, Player, Position, Renderable,
};
use crate::state::State;

fn main() {
    let context =
        Rltk::init_simple8x8(80, 50, "MORTAL WOMBAT", "resources");
    let mut gs = State {
        ecs: World::new(),
        systems: DispatcherBuilder::new()
            .with(LeftWalker {}, "left_walker", &[])
            .build(),
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    gs.ecs.insert(map::new());

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    (0..10).into_iter().for_each(|i| {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: rltk::to_cp437('?'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(LeftMover {})
            .build();
    });

    rltk::main_loop(context, gs);
}

fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}
