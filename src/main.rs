mod engine;

use engine::main_loop;
use engine::state::GameState;
use engine::system_context::context;

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
        .canvas_size(1024, 768)
        .assets_path("assets/")
        .build()
        .unwrap();

    main_loop::run(ctx, &state);
}
