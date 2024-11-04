extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas as SdlCanvas, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CanvasError {
    VideoInit(String),
    WindowInit(sdl2::video::WindowBuildError),
    CanvasInit(sdl2::IntegerOrSdlError),
    FontNotFound(String),
    TextureCreation(String),
    RenderTarget(String),
}

impl fmt::Display for CanvasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CanvasError::VideoInit(e) => write!(f, "Video initialization error: {}", e),
            CanvasError::WindowInit(e) => write!(f, "Window initialization error: {}", e),
            CanvasError::CanvasInit(e) => write!(f, "Canvas initialization error: {}", e),
            CanvasError::FontNotFound(e) => write!(f, "Font not found: {}", e),
            CanvasError::TextureCreation(e) => write!(f, "Failed to create texture: {}", e),
            CanvasError::RenderTarget(e) => write!(f, "Failed to set render target: {}", e),
        }
    }
}

impl Error for CanvasError {}

pub struct Canvas {
    canvas: SdlCanvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
}

impl Canvas {
    pub fn new(
        sdl_context: &Sdl,
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<Self, CanvasError> {
        let video_subsystem = sdl_context.video().map_err(CanvasError::VideoInit)?;
        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .map_err(CanvasError::WindowInit)?;

        let canvas = window
            .into_canvas()
            .build()
            .map_err(CanvasError::CanvasInit)?;

        let texture_creator = canvas.texture_creator();

        Ok(Self {
            canvas,
            texture_creator,
        })
    }

    pub fn draw<F>(&mut self, draw_fn: F) -> Result<(), String>
    where
        F: FnOnce(&mut SdlCanvas<Window>, &TextureCreator<WindowContext>) -> Result<(), String>,
    {
        draw_fn(&mut self.canvas, &self.texture_creator)
    }

    pub fn render_text<'a>(
        &mut self,
        text: &str,
        font: &'a Font<'a, 'static>,
        color: Color,
        x: i32,
        y: i32,
    ) -> Result<(), CanvasError> {
        // Render the text to a surface
        let surface = font
            .render(text)
            .solid(color)
            .map_err(|e| CanvasError::TextureCreation(e.to_string()))?;

        // Create texture from surface
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(surface)
            .map_err(|e| CanvasError::TextureCreation(e.to_string()))?;

        // Get the size of the rendered text
        let (w, h) = font
            .size_of(text)
            .map_err(|e| CanvasError::TextureCreation(e.to_string()))?;

        // Render the texture
        let target = Rect::new(x, y, w, h);
        self.canvas
            .copy(&texture, None, Some(target))
            .map_err(|e| CanvasError::RenderTarget(e.to_string()))?;

        Ok(())
    }
}
