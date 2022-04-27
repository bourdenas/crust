use crate::sdl::Canvas;
use crate::Status;

pub struct SceneManager {
    canvas: Canvas,
}

impl SceneManager {
    pub fn new(canvas: Canvas) -> Self {
        SceneManager { canvas }
    }

    pub fn setup(&mut self) -> Result<(), Status> {
        Ok(())
    }

    /// Renders the scene update since last frame.
    pub fn render(&mut self) {
        // TODO: get all dirty nodes and blit them
        self.canvas.flip();
    }

    /// Renders the whole scene from scratch.
    pub fn render_all(&mut self) {
        self.canvas.clear();
        self.canvas.flip();
    }
}
