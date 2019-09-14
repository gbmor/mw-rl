use std::cmp::{max, min};

use rltk::{Console, RandomNumberGenerator, Rltk, RGB};

use crate::entity::TileType;
use crate::rect::Rect;

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn new_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 * 50];

    (0..80).into_iter().for_each(|x| {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    });

    (0..50).into_iter().for_each(|y| {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    });

    //rando-splat of walls
    let mut rng = rltk::RandomNumberGenerator::new();

    (0..400).into_iter().for_each(|_| {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    });

    map
}

pub fn new_room_corridors() -> (Vec<Rect>, Vec<TileType>) {
    let mut map = vec![TileType::Wall; 80 * 50];

    let mut rooms: Vec<Rect> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    (0..MAX_ROOMS).into_iter().for_each(|_| {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, 80 - w - 1) - 1;
        let y = rng.roll_dice(1, 50 - h - 1) - 1;

        let new_room = Rect::new(x, y, w, h);
        let mut ok = true;
        rooms.iter().for_each(|other| {
            if new_room.intersect(other) {
                ok = false;
            }
        });

        if ok {
            apply_room(&new_room, &mut map);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                if rng.range(0, 1) == 1 {
                    apply_horiz_tunnel(&mut map, prev_x, new_x, prev_y);
                    apply_vert_tunnel(&mut map, prev_y, new_y, new_x);
                } else {
                    apply_vert_tunnel(&mut map, prev_y, new_y, prev_x);
                    apply_horiz_tunnel(&mut map, prev_x, new_x, new_y);
                }
            }
            rooms.push(new_room);
        }
    });

    (rooms, map)
}

pub fn draw(map: &[TileType], ctx: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;
    map.iter().for_each(|tile| {
        match tile {
            TileType::Floor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('.'),
                );
            }
            TileType::Wall => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('#'),
                );
            }
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    });
}

pub fn apply_room(room: &Rect, map: &mut [TileType]) {
    (room.y1 + 1..=room.y2).into_iter().for_each(|y| {
        (room.x1 + 1..=room.x2).into_iter().for_each(|x| {
            map[xy_idx(x, y)] = TileType::Floor;
        });
    });
}

fn apply_horiz_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    (min(x1, x2)..=max(x1, x2)).into_iter().for_each(|x| {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80 * 50 {
            map[idx as usize] = TileType::Floor;
        }
    });
}

fn apply_vert_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    (min(y1, y2)..=max(y1, y2)).into_iter().for_each(|y| {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80 * 50 {
            map[idx as usize] = TileType::Floor;
        }
    });
}
