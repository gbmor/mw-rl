use rltk::{Console, GameState, Rltk, RGB};

struct State {
    y: i32,
    descending: bool,
}

impl GameState for State {
    fn tick(&mut self, context: &mut Rltk) {
        let col1 = RGB::named(rltk::CYAN);
        let col2 = RGB::named(rltk::YELLOW);
        let pct: f32 = self.y as f32 / 50.0;
        let fg = col1.lerp(col2, pct);

        context.cls();
        context.print_color(
            1,
            self.y,
            fg,
            RGB::named(rltk::BLACK),
            "MORTAL WOMBAT MFERS",
        );

        if self.descending {
            self.y += 1;
            if self.y > 48 {
                self.descending = false;
            }
        } else {
            self.y -= 1;
            if self.y < 1 {
                self.descending = true;
            }
        }

        context.draw_box(
            39,
            0,
            20,
            3,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
        );

        context.print_color(
            40,
            2,
            RGB::named(rltk::CYAN),
            RGB::named(rltk::BLACK),
            &format!("Frame time: {}ms", context.frame_time_ms),
        )
    }
}

fn main() {
    let context =
        Rltk::init_simple8x8(80, 50, "Mortal Wombat", "resources");
    let gs = State {
        y: 1,
        descending: true,
    };

    rltk::main_loop(context, gs);
}
