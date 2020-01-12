use rltk::{Console, GameState, Rltk};
use specs::prelude::*;

use crate::entity::{Position, Renderable};
use crate::map;
use crate::player;

pub struct State {
    pub ecs: World,
    pub systems: Dispatcher<'static, 'static>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player::input(self, ctx);
        self.systems.dispatch(&self.ecs);

        map::draw(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<map::Map>();

        (&positions, &renderables).join().for_each(|(pos, rend)| {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, rend.fg, rend.bg, rend.glyph);
            }
        });
    }
}
