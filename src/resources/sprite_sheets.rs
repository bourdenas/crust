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
                    Rect::new(1, 79, 24, 28),
                    Rect::new(33, 78, 24, 29),
                    Rect::new(65, 79, 24, 28),
                    Rect::new(1, 115, 24, 28),
                    Rect::new(33, 114, 24, 29),
                    Rect::new(65, 114, 24, 28),
                ],
            })),
            _ => Err(Status::not_found(&format!(
                "Sprite sheet '{}' was not found.",
                key
            ))),
        }
    }
}

use serde::{de::Visitor, Deserialize, Serialize};
use serde_with::serde_as;

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteSheet {
    pub resource: String,

    #[serde_as(as = "Vec<CrustRect>")]
    pub bounding_boxes: Vec<Rect>,
}

struct CrustRect;

impl serde_with::SerializeAs<Rect> for CrustRect {
    fn serialize_as<S>(source: &Rect, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = format!(
            "[{}, {}, {}, {}]",
            source.x(),
            source.y(),
            source.width(),
            source.height()
        );
        serializer.serialize_str(&s)
    }
}

impl<'de> serde_with::DeserializeAs<'de, Rect> for CrustRect {
    fn deserialize_as<D>(deserializer: D) -> Result<Rect, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CrustRectVisitor;

        impl<'de> Visitor<'de> for CrustRectVisitor {
            type Value = Rect;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a json vector containing exactly 4 integers")
            }
        }

        let s = String::deserialize(deserializer)?;
        println!("value: {}", &s);
        let mut v = vec![];
        for value in s.split(",") {
            match value.parse::<i32>() {
                Ok(value) => v.push(value),
                Err(_) => {
                    return Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(&s),
                        &CrustRectVisitor {},
                    ));
                }
            }
        }

        if v.len() != 4 {
            return Err(serde::de::Error::invalid_length(
                v.len(),
                &CrustRectVisitor {},
            ));
        }

        Ok(Rect::new(v[0], v[1], v[2] as u32, v[3] as u32))
    }
}
