extern crate sdl2;

use sdl2::event::EventPollIterator;
use sdl2::EventPump as SdlEventPump;
use sdl2::Sdl;

pub struct EventPump {
    event_pump: SdlEventPump,
}

impl EventPump {
    pub fn new(sdl_context: &Sdl) -> Result<Self, String> {
        let event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

        Ok(Self { event_pump })
    }

    // TODO: replace this with something that better fits my use case
    pub fn poll_iter(&mut self) -> EventPollIterator {
        self.event_pump.poll_iter()
    }
}
