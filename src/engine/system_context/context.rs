extern crate sdl2;

use sdl2::ttf;
use sdl2::ttf::InitError as Sdl2TtfInitError;
use std::error::Error;
use std::fmt;

use super::asset_loader::AssetLoader;
use super::canvas::{Canvas, CanvasError};
use super::events::EventPump;

#[derive(Debug)]
pub enum ContextError {
    SdlInit(String),
    TtfInit(Sdl2TtfInitError),
    CanvasInit(CanvasError),
    EventsInit(String),
}

impl fmt::Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContextError::SdlInit(e) => write!(f, "SDL initialization error: {}", e),
            ContextError::TtfInit(e) => write!(f, "TTF initialization error: {}", e),
            ContextError::CanvasInit(e) => write!(f, "Canvas initialization error: {}", e),
            ContextError::EventsInit(e) => write!(f, "EventPump initialization error: {}", e),
        }
    }
}

impl Error for ContextError {}

#[derive(Debug)]
pub struct ContextConfig {
    canvas_title: String,
    canvas_width: u32,
    canvas_height: u32,
    assets_path: Option<String>,
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            canvas_title: "window".to_string(),
            canvas_width: 800,
            canvas_height: 600,
            assets_path: None,
        }
    }
}

pub struct ContextBuilder {
    config: ContextConfig,
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self {
            config: ContextConfig::default(),
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.config.canvas_title = title.to_string();
        self
    }

    pub fn canvas_size(mut self, width: u32, height: u32) -> Self {
        self.config.canvas_width = width;
        self.config.canvas_height = height;
        self
    }

    pub fn assets_path(mut self, path: &str) -> Self {
        self.config.assets_path = Some(path.to_string());
        self
    }

    pub fn build(self) -> Result<Context, ContextError> {
        Context::new(self.config)
    }
}

pub struct Context {
    _sdl_context: sdl2::Sdl,

    canvas: Canvas,
    asset_loader: AssetLoader,
    event_pump: EventPump,
}

impl Context {
    fn new(config: ContextConfig) -> Result<Self, ContextError> {
        let sdl_context = sdl2::init().map_err(ContextError::SdlInit)?;
        let ttf_context = ttf::init().map_err(ContextError::TtfInit)?;

        let canvas = Canvas::new(
            &sdl_context,
            &config.canvas_title,
            config.canvas_width,
            config.canvas_height,
        )
        .map_err(ContextError::CanvasInit)?;

        let event_pump =
            EventPump::new(&sdl_context).map_err(|e| ContextError::EventsInit(e.to_string()))?;

        let asset_loader = AssetLoader::new(ttf_context);

        Ok(Self {
            _sdl_context: sdl_context,
            canvas,
            asset_loader,
            event_pump,
        })
    }

    pub fn gfx<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Canvas, &mut AssetLoader) -> R,
    {
        f(&mut self.canvas, &mut self.asset_loader)
    }

    pub fn poll_events<F>(&mut self, f: F)
    where
        F: FnOnce(&mut EventPump),
    {
        f(&mut self.event_pump)
    }
}
