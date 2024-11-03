extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

pub struct Context {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
}

pub struct ContextBuilder<'a> {
    title: Option<&'a str>,
    canvas_width: Option<u32>,
    canvas_height: Option<u32>,
}

impl<'a> ContextBuilder<'a> {
    pub fn title(&mut self, title: &'a str) -> &mut Self {
        self.title = Some(title);

        self
    }

    pub fn dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.canvas_width = Some(width);
        self.canvas_height = Some(height);

        self
    }

    pub fn build(&self) -> Context {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(
                self.title.unwrap_or("window"),
                self.canvas_width.unwrap_or(800),
                self.canvas_height.unwrap_or(600),
            )
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Context {
            sdl_context,
            canvas,
        }
    }
}

impl<'a> Context {
    pub fn context() -> ContextBuilder<'a> {
        ContextBuilder {
            title: None,
            canvas_width: None,
            canvas_height: None,
        }
    }
}
