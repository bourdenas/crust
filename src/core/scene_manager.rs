use super::Status;

pub struct SceneManager;

impl SceneManager {
    pub fn new() -> Self {
        SceneManager {}
    }

    pub fn load(&mut self, resource: &str) -> Result<(), Status> {
        todo!("")
    }
}
