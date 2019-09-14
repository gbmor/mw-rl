#[macro_use]
extern crate specs_derive;

use rltk::{Console, GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;

#[derive(Debug, Component)]
struct Player {}

#[derive(Debug, Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: u8,
    fg: RGB,
    bg: RGB,
}

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

#[derive(Component)]
struct LeftMover {}

struct LeftWalker {}
impl<'a> System<'a> for LeftWalker {
    type SystemData =
        (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= -1;
            pos.y -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
            if pos.x > 79 {
                pos.x = 0;
            }
            if pos.y < 0 {
                pos.y = 49;
            }
            if pos.y > 49 {
                pos.y = 0;
            }
        }
    }
}

struct State {
    ecs: World,
    systems: Dispatcher<'static, 'static>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.systems.dispatch(&self.ecs);

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, rend) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, rend.fg, rend.bg, rend.glyph);
        }
    }
}

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

    gs.ecs.insert(new_map());

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

fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
    let mut posns = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    (&mut players, &mut posns).join().into_iter().for_each(
        |(_player, pos)| {
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
        },
    );
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}

fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

fn new_map() -> Vec<TileType> {
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

fn draw_map(map: &[TileType], ctx: &mut Rltk) {
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
