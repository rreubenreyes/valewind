extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::TextureQuery;
use sdl2::surface::Surface;
use sdl2::ttf;
use sdl2::ttf::Font;
use sdl2::ttf::FontStyle;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use sdl2::EventPump;
use std::error::Error;
use std::fmt;

pub struct Context {
    sdl_context: sdl2::Sdl,
    events: EventPump,
    canvas: Canvas<Window>,
    ttf: Sdl2TtfContext,
}

pub struct ContextBuilder<'a> {
    title: &'a str,
    canvas_width: u32,
    canvas_height: u32,
}

#[derive(Debug)]
pub enum ContextError {
    SdlInit(String),
    VideoInit(String),
    WindowInit(sdl2::video::WindowBuildError),
    CanvasInit(sdl2::IntegerOrSdlError),
    TtfInit(sdl2::ttf::InitError),
    EventInit(String),
}

impl fmt::Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContextError::SdlInit(e) => write!(f, "SDL initialization error: {}", e),
            ContextError::VideoInit(e) => write!(f, "Video initialization error: {}", e),
            ContextError::WindowInit(e) => write!(f, "Window initialization error: {}", e),
            ContextError::CanvasInit(e) => write!(f, "Canvas initialization error: {}", e),
            ContextError::TtfInit(e) => write!(f, "TTF initialization error: {}", e),
            ContextError::EventInit(e) => write!(f, "Event initialization error: {}", e),
        }
    }
}

impl Error for ContextError {}

impl Default for ContextBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ContextBuilder<'a> {
    pub fn new() -> Self {
        Self {
            title: "window",
            canvas_width: 800,
            canvas_height: 600,
        }
    }

    pub fn title(&mut self, title: &'a str) -> &mut Self {
        self.title = title;

        self
    }

    pub fn dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.canvas_width = width;
        self.canvas_height = height;

        self
    }

    pub fn build(&self) -> Result<Context, ContextError> {
        let sdl_context = sdl2::init().map_err(|e| ContextError::SdlInit(e))?;

        let video_subsystem = sdl_context
            .video()
            .map_err(|e| ContextError::VideoInit(e))?;

        let window = video_subsystem
            .window(self.title, self.canvas_width, self.canvas_height)
            .position_centered()
            .build()
            .map_err(|e| ContextError::WindowInit(e))?;

        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| ContextError::CanvasInit(e))?;

        let events = sdl_context
            .event_pump()
            .map_err(|e| ContextError::EventInit(e))?;

        let ttf = ttf::init().map_err(|e| ContextError::TtfInit(e))?;

        Ok(Context {
            sdl_context,
            events,
            canvas,
            ttf,
        })
    }
}

impl Context {
    pub fn events(&mut self) -> &mut EventPump {
        &mut self.events
    }

    pub fn canvas(&mut self) -> &mut Canvas<Window> {
        &mut self.canvas
    }

    pub fn load_font(&self, path: &str) -> Result<Font, String> {
        let mut font = self.ttf.load_font(path, 32)?;
        font.set_style(FontStyle::NORMAL);

        Ok(font)
    }

    // TODO: Handle SDL2 entity creation separately, "send" to texture creator/etc
    pub fn render_texture(&mut self, surface: &Surface, x: i32, y: i32) -> Result<(), String> {
        let canvas = self.canvas();
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new(x, y, width, height);

        self.canvas.copy(&texture, None, Some(target))?;

        Ok(())
    }
}
