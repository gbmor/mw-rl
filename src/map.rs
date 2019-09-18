use std::cmp::{max, min};

use rltk::{
    Algorithm2D, BaseMap, Console, Point, RandomNumberGenerator, Rltk,
    RGB,
};
use specs::prelude::*;

use crate::component::Viewshed;
use crate::entity::{Player, TileType};
use crate::rect::Rect;

#[derive(Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
}

impl Algorithm2D for Map {
    fn point2d_to_index(&self, pt: Point) -> i32 {
        (pt.y * self.width) + pt.x
    }
    fn index_to_point2d(&self, idx: i32) -> Point {
        Point {
            x: idx % self.width,
            y: idx / self.width,
        }
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: i32) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }
    fn get_available_exits(&self, _idx: i32) -> Vec<(i32, f32)> {
        Vec::new()
    }
    fn get_pathing_distance(&self, idx1: i32, idx2: i32) -> f32 {
        let p1 = Point::new(idx1 % self.width, idx1 / self.width);
        let p2 = Point::new(idx2 % self.width, idx2 / self.width);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * 80) + x as usize
    }

    pub fn apply_room(&mut self, room: &Rect) {
        (room.y1 + 1..=room.y2).into_iter().for_each(|y| {
            (room.x1 + 1..=room.x2).into_iter().for_each(|x| {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            });
        });
    }

    pub fn apply_horiz_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        (min(x1, x2)..=max(x1, x2)).into_iter().for_each(|x| {
            let idx = self.xy_idx(x, y);
            if idx > 0
                && idx < self.width as usize * self.height as usize
            {
                self.tiles[idx as usize] = TileType::Floor;
            }
        });
    }

    pub fn apply_vert_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        (min(y1, y2)..=max(y1, y2)).into_iter().for_each(|y| {
            let idx = self.xy_idx(x, y);
            if idx > 0
                && idx < self.width as usize * self.height as usize
            {
                self.tiles[idx as usize] = TileType::Floor;
            }
        });
    }

    pub fn new_room_corridors() -> Self {
        let mut map = Self {
            tiles: vec![TileType::Wall; 80 * 50],
            rooms: vec![],
            width: 80,
            height: 50,
        };

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
            map.rooms.iter().for_each(|other| {
                if new_room.intersect(other) {
                    ok = false;
                }
            });

            if ok {
                map.apply_room(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) =
                        map.rooms[map.rooms.len() - 1].center();

                    if rng.range(0, 1) == 1 {
                        map.apply_horiz_tunnel(prev_x, new_x, prev_y);
                        map.apply_vert_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vert_tunnel(prev_y, new_y, prev_x);
                        map.apply_horiz_tunnel(prev_x, new_x, new_y);
                    }
                }
                map.rooms.push(new_room);
            }
        });

        map
    }
}

/*
pub fn new_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 * 50];

    (0..80).into_iter().for_each(|x| {
        map[Map::xy_idx(x, 0)] = TileType::Wall;
        map[Map::xy_idx(x, 49)] = TileType::Wall;
    });

    (0..50).into_iter().for_each(|y| {
        map[Map::xy_idx(0, y)] = TileType::Wall;
        map[Map::xy_idx(79, y)] = TileType::Wall;
    });

    //rando-splat of walls
    let mut rng = rltk::RandomNumberGenerator::new();

    (0..400).into_iter().for_each(|_| {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = Map::xy_idx(x, y);
        if idx != Map::xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    });

    map
}
*/

pub fn draw(ecs: &World, ctx: &mut Rltk) {
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    (&mut players, &mut viewsheds).join().into_iter().for_each(
        |(_player, viewshed)| {
            let mut y = 0;
            let mut x = 0;
            map.tiles.iter().for_each(|tile| {
                let pt = Point::new(x, y);
                if viewshed.visible_tiles.contains(&pt) {
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
                }

                x += 1;
                if x > 79 {
                    x = 0;
                    y += 1;
                }
            });
        },
    );
}
