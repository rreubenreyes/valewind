mod engine;

use engine::context;
use engine::main_loop;
use engine::state::GameState;

struct State {}

impl GameState for State {
    // this updates the client side state somehow
    fn tick(&self) {
        print!("Hello Valewind")
    }
}

fn main() {
    // init game state
    let state = State {};

    let ctx = context::ContextBuilder::new()
        .title("Valewind")
        .dimensions(800, 600)
        .build()
        .unwrap(); // TODO: handle context init error more explicitly

    main_loop::run(ctx, &state);
}
