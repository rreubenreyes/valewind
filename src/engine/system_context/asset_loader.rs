use sdl2::ttf::{Font, FontStyle, Sdl2TtfContext};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AssetLoaderError {
    LoadFont(String),
    AssetNotFound(String),
}

impl fmt::Display for AssetLoaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssetLoaderError::LoadFont(e) => write!(f, "Error loading font: {}", e),
            AssetLoaderError::AssetNotFound(name) => write!(f, "Asset not found: {}", name),
        }
    }
}

impl Error for AssetLoaderError {}

pub struct AssetLoader {
    fonts: HashMap<String, Font<'static, 'static>>,
}

impl AssetLoader {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
        }
    }

    pub fn get_font(
        &mut self,
        ttf_context: &Sdl2TtfContext,
        name: &str,
        path: &str,
        size: u16,
    ) -> Result<&Font<'static, 'static>, AssetLoaderError> {
        if !self.fonts.contains_key(name) {
            let mut font = ttf_context
                .load_font(path, size)
                .map_err(|e| AssetLoaderError::LoadFont(e.to_string()))?;

            font.set_style(FontStyle::NORMAL);

            // Extend the lifetime of the font
            let font = unsafe { std::mem::transmute::<Font<'_, '_>, Font<'static, 'static>>(font) };

            self.fonts.insert(name.to_string(), font);
        }

        self.fonts
            .get(name)
            .ok_or_else(|| AssetLoaderError::AssetNotFound(name.to_string()))
    }
}
