use crate::engine::context::Context;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;

pub fn render(ctx: &mut Context, text: &str, x: i32, y: i32) -> Result<(), String> {
    ctx.draw(|canvas, assets, texture_creator| {
        let font = assets
            .get_font("default")
            .ok_or_else(|| format!("Font not found: {}", "default"))?;

        let surface = font
            .render(text)
            .blended(Color::RGB(255, 255, 255))
            .map_err(|e| e.to_string())?;

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new(x, y, width, height);

        canvas.copy(&texture, None, Some(target))?;

        Ok(())
    })
}
