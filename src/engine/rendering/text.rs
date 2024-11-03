use crate::engine::context::Context;

use sdl2::pixels::Color;

pub fn render(ctx: &mut Context, text: &str) -> Result<(), String> {
    // Create the surface in a separate scope so the font borrow ends
    let surface = {
        let font = ctx
            // TODO: handle font selection somewhere else rather than hardcoding Dank Mono
            .load_font("/Users/chroma/Library/Fonts/DankMonoNerdFont-Regular.ttf")
            .unwrap();

        font.render(text)
            .blended(Color::RGB(255, 255, 255))
            .unwrap()
    }; // font borrow ends here

    // Now we can borrow ctx again
    ctx.render_texture(&surface, 10, 10)?;

    Ok(())
}
