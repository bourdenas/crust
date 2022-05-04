use crate::core::Status;
use sdl2::rect::Rect;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

pub struct SpriteSheetsManager {
    _cache: Arc<HashMap<String, Arc<SpriteSheet>>>,
}

impl SpriteSheetsManager {
    pub fn new() -> Self {
        SpriteSheetsManager {
            _cache: Arc::new(HashMap::new()),
        }
    }

    pub fn load(&self, key: &str) -> Result<Rc<SpriteSheet>, Status> {
        // TODO: Eventually should load them from disk.
        match key {
            "reaper" => Ok(Rc::new(SpriteSheet {
                resource: "reaper".to_owned(),
                bounding_boxes: vec![
                    Rect::new(6, 7, 24, 28),
                    Rect::new(37, 6, 24, 29),
                    Rect::new(70, 6, 24, 28),
                    Rect::new(5, 43, 24, 28),
                    Rect::new(37, 42, 24, 29),
                    Rect::new(68, 43, 25, 28),
                ],
            })),
            _ => Err(Status::not_found(&format!(
                "Sprite sheet '{}' was not found.",
                key
            ))),
        }
    }
}

pub struct SpriteSheet {
    pub resource: String,
    pub bounding_boxes: Vec<Rect>,
}
