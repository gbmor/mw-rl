use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;

use crate::entity::{Player, Position, TileType};
use crate::map::Map;
use crate::state::State;

pub fn try_move(dx: i32, dy: i32, ecs: &mut World) {
    let mut posns = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    (&mut players, &mut posns).join().into_iter().for_each(
        |(_player, pos)| {
            let dest_idx = map.xy_idx(pos.x + dx, pos.y + dy);
            if map.tiles[dest_idx] != TileType::Wall {
                pos.x += dx;
                pos.y += dy;

                if pos.x < 0 {
                    pos.x = 0;
                }
                if pos.x > 79 {
                    pos.x = 79;
                }
                if pos.y < 0 {
                    pos.y = 0;
                }
                if pos.y > 49 {
                    pos.y = 49;
                }
            }
        },
    );
}

pub fn input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Numpad4 => try_move(-1, 0, &mut gs.ecs),
            VirtualKeyCode::H => try_move(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move(1, 0, &mut gs.ecs),
            VirtualKeyCode::Numpad6 => try_move(1, 0, &mut gs.ecs),
            VirtualKeyCode::L => try_move(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move(0, -1, &mut gs.ecs),
            VirtualKeyCode::Numpad8 => try_move(0, -1, &mut gs.ecs),
            VirtualKeyCode::K => try_move(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move(0, 1, &mut gs.ecs),
            VirtualKeyCode::Numpad2 => try_move(0, 1, &mut gs.ecs),
            VirtualKeyCode::J => try_move(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}
