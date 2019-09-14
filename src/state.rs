use rltk::{Console, GameState, Rltk};
use specs::prelude::*;

use crate::entity::{Position, Renderable, TileType};
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

        let map = self.ecs.fetch::<Vec<TileType>>();
        map::draw(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, rend) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, rend.fg, rend.bg, rend.glyph);
        }
    }
}
