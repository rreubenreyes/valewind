extern crate sdl2;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf;
use sdl2::ttf::{Font, FontStyle, Sdl2TtfContext};
use sdl2::video::{Window, WindowContext};
use sdl2::EventPump;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

pub struct AssetCache {
    fonts: HashMap<String, Font<'static, 'static>>,
}

impl AssetCache {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
        }
    }

    pub fn load_font(
        &mut self,
        context: &Sdl2TtfContext,
        name: &str,
        path: &str,
        size: u16,
    ) -> Result<(), String> {
        let mut font = context.load_font(path, size)?;
        font.set_style(FontStyle::NORMAL);

        // Convert the font to 'static since it's tied to the Arc'd context
        let font: Font<'static, 'static> = unsafe { std::mem::transmute(font) };
        self.fonts.insert(name.to_string(), font);
        Ok(())
    }

    pub fn get_font(&self, name: &str) -> Option<&Font> {
        self.fonts.get(name)
    }
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

        let texture_creator = canvas.texture_creator();

        let events = sdl_context
            .event_pump()
            .map_err(|e| ContextError::EventInit(e))?;

        let ttf = Arc::new(ttf::init().map_err(|e| ContextError::TtfInit(e))?);

        Ok(Context {
            sdl_context,
            events,
            canvas,
            ttf,
            texture_creator,
            assets: AssetCache::new(),
        })
    }
}

pub struct Context {
    sdl_context: sdl2::Sdl,
    events: EventPump,
    canvas: Canvas<Window>,
    ttf: Arc<Sdl2TtfContext>,
    texture_creator: TextureCreator<WindowContext>,
    assets: AssetCache,
}

impl Context {
    pub fn initialize(mut self) -> Result<Self, String> {
        self.initialize_fonts()?;

        Ok(self)
    }

    pub fn events(&mut self) -> &mut EventPump {
        &mut self.events
    }

    fn initialize_fonts(&mut self) -> Result<(), String> {
        self.assets.load_font(
            &self.ttf,
            "default",
            "/Users/chroma/Library/Fonts/DankMonoNerdFont-Regular.ttf",
            32,
        )?;

        Ok(())
    }

    pub fn draw<F>(&mut self, draw_fn: F) -> Result<(), String>
    where
        F: FnOnce(
            &mut Canvas<Window>,
            &mut AssetCache,
            &TextureCreator<WindowContext>,
        ) -> Result<(), String>,
    {
        draw_fn(&mut self.canvas, &mut self.assets, &self.texture_creator)
    }
}
