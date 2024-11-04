use sdl2::ttf::{FontStyle, Sdl2TtfContext};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum AssetLoaderError {
    CouldNotRegister(String),
    CouldNotLoad(String),
    NotRegistered(String),
}

impl fmt::Display for AssetLoaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssetLoaderError::CouldNotRegister(e) => {
                write!(f, "error while registering font {}", e)
            }
            AssetLoaderError::CouldNotLoad(e) => write!(f, "error while loading font {}", e),
            AssetLoaderError::NotRegistered(name) => {
                write!(f, "tried to load unregistered asset: {}", name)
            }
        }
    }
}

impl Error for AssetLoaderError {}

// INFO: store the _parameters_ needed to create the font, rather than the font itself
// font lifetime is not tied to `ttf_context` passed to `AssetLoader` or any particular struct,
// for that matter
#[derive(Clone)]
struct FontCacheEntry {
    path: PathBuf,
    size: u16,
    style: FontStyle,
}

pub struct AssetLoader {
    ttf_context: Sdl2TtfContext,
    font_cache: HashMap<String, FontCacheEntry>,
}

impl AssetLoader {
    pub fn new(ttf_context: Sdl2TtfContext) -> Self {
        Self {
            ttf_context,
            font_cache: HashMap::new(),
        }
    }

    pub fn register_font(
        &mut self,
        name: &str,
        path: impl AsRef<Path>,
        size: u16,
    ) -> Result<(), AssetLoaderError> {
        self.ttf_context
            .load_font(&path, size)
            .map_err(|e| AssetLoaderError::CouldNotRegister(e.to_string()))?;

        let entry = FontCacheEntry {
            path: path.as_ref().to_path_buf(),
            size,
            style: FontStyle::NORMAL,
        };

        self.font_cache.insert(name.to_string(), entry);
        Ok(())
    }

    // Load a registered font
    pub fn load_font<'a>(
        &'a self,
        name: &str,
    ) -> Result<sdl2::ttf::Font<'a, 'static>, AssetLoaderError> {
        let entry = self
            .font_cache
            .get(name)
            .ok_or_else(|| AssetLoaderError::NotRegistered(name.to_string()))?;

        // INFO: sdl2_ttf font is loaded here rather than loaded from a cache;
        // we don't need to deal with this struct's lifetime this way
        //
        // XXX: however, this will still load the font from disk every time we call it - might want to
        // consider caching or something to improve performance later
        let mut font = self
            .ttf_context
            .load_font(&entry.path, entry.size)
            .map_err(|e| AssetLoaderError::CouldNotLoad(e.to_string()))?;

        font.set_style(entry.style);
        Ok(font)
    }
}
