use crate::sdl::texture::TextureLoader;
use crate::Status;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

pub struct Canvas {
    canvas: WindowCanvas,
}

impl Canvas {
    pub fn new(
        video_subsystem: &sdl2::VideoSubsystem,
        title: &str,
        width: u32,
        height: u32,
    ) -> Self {
        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");

        Canvas {
            canvas: window
                .into_canvas()
                .build()
                .expect("could not make a canvas"),
        }
    }

    pub fn create_texture_loader(&self) -> TextureLoader {
        TextureLoader::new(&self.canvas)
    }

    pub fn blit<R1, R2>(&mut self, texture: &Texture, src: R1, dst: R2) -> Result<(), Status>
    where
        R1: Into<Option<Rect>>,
        R2: Into<Option<Rect>>,
    {
        self.canvas.copy(texture, src, dst)?;
        Ok(())
    }

    pub fn fill_rect<R: Into<Option<Rect>>>(&mut self, color: Color, dst: R) -> Result<(), Status> {
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(dst)?;
        Ok(())
    }

    pub fn flip(&mut self) {
        self.canvas.present();
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }
}
