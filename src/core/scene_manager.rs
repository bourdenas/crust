use crate::Status;
use sdl2::render::Texture;
use sdl2::render::WindowCanvas;

pub struct SceneManager;

impl SceneManager {
    pub fn new() -> Self {
        SceneManager {}
    }

    pub fn setup(&mut self) -> Result<(), Status> {
        Ok(())
    }

    /// Renders the scene update since last frame.
    pub fn render(&mut self, canvas: &mut WindowCanvas, texture: &Texture) {
        // TODO: get all dirty nodes and blit them
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
    }

    /// Renders the whole scene from scratch.
    pub fn render_all(&mut self, canvas: &mut WindowCanvas) {
        canvas.clear();
        canvas.present();
    }
}
