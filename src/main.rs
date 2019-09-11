use rltk::{Console, GameState, Rltk};

struct State {}
impl GameState for State {
    fn tick(&mut self, context: &mut Rltk) {
        context.cls();
        context.print(1, 1, "MORTAL WOMBAT MFERS");
    }
}

fn main() {
    let context =
        Rltk::init_simple8x8(80, 50, "Mortal Wombat", "resources");
    let gamestate = State {};
    rltk::main_loop(context, gamestate);
}
