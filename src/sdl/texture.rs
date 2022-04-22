use crate::Status;
use sdl2;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, WindowCanvas};

pub struct TextureLoader {
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
}

impl TextureLoader {
    pub fn new(canvas: &WindowCanvas) -> Self {
        TextureLoader {
            texture_creator: canvas.texture_creator(),
        }
    }

    pub fn load_texture(&self, path: &str) -> Result<Texture, Status> {
        Ok(self.texture_creator.load_texture(path)?)
    }
}
