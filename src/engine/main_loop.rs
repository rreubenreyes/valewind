use super::state::GameState;
use super::system_context::context::Context;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub fn run(mut ctx: Context, _state: &impl GameState) {
    let mut i = 0;
    'running: loop {
        // XXX: this is all test code to test out asset loading and rendering
        i = (i + 1) % 255;

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

        ctx.gfx(|canvas, asset_loader| {
            // TODO: do asset registration somewhere else; probably want
            // `AssetLoader.register(config)` or something
            let _ = asset_loader.register_font(
                "normal",
                "/Users/chroma/Library/Fonts/DankMonoNerdFont-Regular.ttf",
                24,
            );

            let font = asset_loader.load_font("normal").unwrap();

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
                .render_text(
                    &format!("Hello, Valewind {}", i),
                    &font,
                    Color::RGB(255, 255, 255),
                    100,
                    100,
                )
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
