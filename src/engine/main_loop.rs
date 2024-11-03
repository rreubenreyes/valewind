use super::context::Context;
use super::rendering;
use super::state::GameState;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub fn run(mut ctx: Context, _state: &impl GameState) {
    // XXX: boilerplate
    let mut i = 0;
    'running: loop {
        // XXX: boilerplate
        i = (i + 1) % 255;

        // XXX: boilerplate; this should eventually be sent as a tick from the passed state
        // TODO: draw from scene manager
        {
            let canvas = ctx.canvas();
            canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            canvas.clear();
        };

        // TODO: handle input from context
        {
            for event in ctx.events().poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
        }

        // XXX: testing text rendering
        rendering::text::render(&mut ctx, &format!("Hello, Valewind {}", i)).unwrap();

        // The rest of the game loop goes here...

        // render at 60 FPS
        ctx.canvas().present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
