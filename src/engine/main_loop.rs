use super::state::GameState;
use super::system_context::context::Context;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub fn run(mut ctx: Context, _state: &impl GameState) {
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;

        // Handle events with a closure
        let mut quit = false;
        ctx.poll_events(|events| {
            for event in events.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        quit = true;
                        break;
                    }
                    _ => {}
                }
            }
        });

        if quit {
            break 'running;
        }

        // Handle all graphics operations in a single closure
        ctx.gfx(|canvas, asset_loader, ttf_context| {
            // font via asset loader
            let font = asset_loader
                .get_font(
                    ttf_context,
                    "default",
                    "/Users/chroma/Library/Fonts/DankMonoNerdFont-Regular.ttf",
                    16,
                )
                .unwrap();

            // colors
            canvas
                .draw(|c, _| {
                    c.set_draw_color(Color::RGB(i, 64, 255 - i));
                    c.clear();
                    Ok(())
                })
                .unwrap();

            // testing text rendering
            canvas
                .render_text("Hello, Valewind", font, Color::RGB(255, 255, 255), 100, 100)
                .unwrap();

            canvas
                .draw(|c, _| {
                    c.present();
                    Ok(())
                })
                .unwrap();
        });

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
