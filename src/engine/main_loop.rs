use super::context;
use super::state::GameState;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub fn run(mut ctx: context::Context, state: &impl GameState) {
    let mut event_pump = ctx.sdl_context.event_pump().unwrap();
    // XXX: boilerplate
    let mut i = 0;
    'running: loop {
        // XXX: boilerplate
        i = (i + 1) % 255;

        // XXX: boilerplate; this should eventually be sent as a tick from the passed state
        // TODO: draw from scene manager
        ctx.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        ctx.canvas.clear();

        // TODO: handle input from context
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // The rest of the game loop goes here...

        // render at 60 FPS
        ctx.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
