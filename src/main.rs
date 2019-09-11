use rand::Rng;
use rltk::{
    Algorithm2D, BaseMap, Console, GameState, Point, Rltk,
    VirtualKeyCode, RGB,
};

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

struct State {
    map: Vec<TileType>,
    player_position: usize,
    visible: Vec<bool>,
}

fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

fn idx_xy(idx: usize) -> (i32, i32) {
    (idx as i32 % 80, idx as i32 / 80)
}

impl State {
    fn new() -> State {
        let mut state = State {
            map: vec![TileType::Floor; 80 * 50],
            player_position: xy_idx(40, 25),
            visible: vec![false; 80 * 50],
        };

        (0..80).into_iter().for_each(|x| {
            state.map[xy_idx(x, 0)] = TileType::Wall;
            state.map[xy_idx(x, 49)] = TileType::Wall;
        });

        (0..50).into_iter().for_each(|y| {
            state.map[xy_idx(0, y)] = TileType::Wall;
            state.map[xy_idx(79, y)] = TileType::Wall;
        });

        let mut rng = rand::thread_rng();

        (0..400).into_iter().for_each(|_| {
            let x = rng.gen_range(1, 79);
            let y = rng.gen_range(1, 49);
            let idx = xy_idx(x, y);

            if state.player_position != idx {
                state.map[idx] = TileType::Wall;
            }
        });

        state
    }

    fn move_player(&mut self, dx: i32, dy: i32) {
        let cur_pos = idx_xy(self.player_position);
        let new_pos = (cur_pos.0 + dx, cur_pos.1 + dy);
        let new_idx = xy_idx(new_pos.0, new_pos.1);
        if self.map[new_idx] == TileType::Floor {
            self.player_position = new_idx;
        }
    }
}

impl GameState for State {
    #[allow(non_snake_case)]
    fn tick(&mut self, ctx: &mut Rltk) {
        match ctx.key {
            None => {}
            Some(key) => {
                match key {
                    //numpad
                    VirtualKeyCode::Numpad8 => self.move_player(0, -1),
                    VirtualKeyCode::Numpad4 => self.move_player(-1, 0),
                    VirtualKeyCode::Numpad6 => self.move_player(1, 0),
                    VirtualKeyCode::Numpad2 => self.move_player(0, 1),

                    //diag
                    VirtualKeyCode::Numpad7 => self.move_player(-1, -1),
                    VirtualKeyCode::Numpad9 => self.move_player(1, -1),
                    VirtualKeyCode::Numpad1 => self.move_player(-1, 1),
                    VirtualKeyCode::Numpad3 => self.move_player(1, 1),

                    //arrows
                    VirtualKeyCode::Up => self.move_player(0, -1),
                    VirtualKeyCode::Down => self.move_player(0, 1),
                    VirtualKeyCode::Left => self.move_player(-1, 0),
                    VirtualKeyCode::Right => self.move_player(1, 0),
                    _ => {}
                }
            }
        }

        self.visible.iter_mut().for_each(|v| {
            *v = false;
        });

        let player_position =
            self.index_to_point2d(self.player_position as i32);
        let fov = rltk::field_of_view(player_position, 8, self);

        fov.iter().for_each(|idx| {
            self.visible[xy_idx(idx.x, idx.y)] = true;
        });

        ctx.cls();

        let mut y = 0;
        let mut x = 0;
        &self.map.iter().enumerate().for_each(|(i, tile)| {
            let mut fg;
            let mut glyph = ".";

            match tile {
                TileType::Floor => {
                    fg = RGB::from_f32(0.5, 0.5, 0.0);
                }
                TileType::Wall => {
                    fg = RGB::from_f32(0.0, 1.0, 0.0);
                    glyph = "#";
                }
            }

            if !self.visible[i] {
                fg = fg.to_greyscale();
            }
            ctx.print_color(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
            x += 1;
            if x > 79 {
                x = 0;
                y += 1;
            }
        });

        let ppos = idx_xy(self.player_position);
        ctx.print_color(
            ppos.0,
            ppos.1,
            RGB::from_f32(1.0, 1.0, 0.0),
            RGB::from_f32(0., 0., 0.),
            "@",
        );
    }
}

impl BaseMap for State {
    fn is_opaque(&self, idx: i32) -> bool {
        self.map[idx as usize] == TileType::Wall
    }
    fn get_available_exits(&self, _idx: i32) -> Vec<(i32, f32)> {
        Vec::new() // no exits? it's a trap!!!
    }
    fn get_pathing_distance(&self, _idx1: i32, _idx2: i32) -> f32 {
        0.0
    }
}

impl Algorithm2D for State {
    fn point2d_to_index(&self, pt: Point) -> i32 {
        xy_idx(pt.x, pt.y) as i32
    }
    fn index_to_point2d(&self, idx: i32) -> Point {
        Point::new(idx % 80, idx / 80)
    }
}

fn main() {
    let context =
        Rltk::init_simple8x8(80, 50, "Mortal Wombat", "resources");
    let gs = State::new();
    rltk::main_loop(context, gs);
}
