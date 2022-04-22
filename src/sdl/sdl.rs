use crate::sdl::Canvas;
use crate::Status;
use sdl2::image::{self, InitFlag};

pub struct Sdl {
    pub sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    _image_context: sdl2::image::Sdl2ImageContext,
}

impl Sdl {
    pub fn init() -> Result<Self, Status> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        Ok(Sdl {
            sdl_context,
            video_subsystem,
            _image_context: image::init(InitFlag::PNG | InitFlag::JPG)?,
        })
    }

    pub fn create_canvas(&self, title: &str, width: u32, height: u32) -> Canvas {
        Canvas::new(&self.video_subsystem, title, width, height)
    }
}
