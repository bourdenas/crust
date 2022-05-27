use crate::core::Status;
use sdl2::rect::Rect;
use std::collections::HashMap;

pub struct SpriteSheetsManager {
    _path: String,
    _cache: HashMap<String, SpriteSheet>,
}

impl SpriteSheetsManager {
    pub fn new(resource_path: &str) -> Self {
        SpriteSheetsManager {
            _path: resource_path.to_owned(),
            _cache: HashMap::new(),
        }
    }

    pub fn load<'a>(&'a mut self, key: &str) -> Result<&'a SpriteSheet, Status> {
        // if let Some(sheet) = self._cache.get(key) {
        //     return Ok(sheet);
        // }

        let filename = format!("{}/{}.json", self._path, key);
        let json = std::fs::read(filename)?;
        match serde_json::from_slice::<SpriteSheet>(&json) {
            Ok(sheet) => {
                self._cache.insert(key.to_owned(), sheet);
                Ok(&self._cache.get(key).unwrap())
            }
            Err(e) => Err(Status::new("Failed to load sprite sheet: {}", e)),
        }
    }
}

use serde::{
    de::{SeqAccess, Visitor},
    Deserialize, Serialize,
};
use serde_with::serde_as;

#[serde_with::serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
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

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut v: Vec<i32> = vec![];
                for _ in 0..4 {
                    match seq.next_element()? {
                        Some(value) => v.push(value),
                        None => {
                            return Err(serde::de::Error::invalid_length(
                                v.len(),
                                &CrustRectVisitor {},
                            ))
                        }
                    }
                }

                match v.len() {
                    4 => Ok(Rect::new(v[0], v[1], v[2] as u32, v[3] as u32)),
                    _ => Err(serde::de::Error::invalid_length(
                        v.len(),
                        &CrustRectVisitor {},
                    )),
                }
            }
        }

        deserializer.deserialize_seq(CrustRectVisitor {})
    }
}
