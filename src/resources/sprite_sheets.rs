use super::{ResourceLoader, ResourceManager};
use crate::core::Status;
use sdl2::rect::Rect;

pub type SpriteSheetsManager = ResourceManager<String, SpriteSheet, SpriteSheetsLoader>;

pub struct SpriteSheetsLoader;

impl ResourceLoader<SpriteSheet> for SpriteSheetsLoader {
    type Args = str;

    fn load(&self, path: &str, resource: &str) -> Result<SpriteSheet, Status> {
        let filename = format!("{}/{}.json", path, resource);
        let json = std::fs::read(filename)?;
        match serde_json::from_slice::<SpriteSheet>(&json) {
            Ok(sheet) => Ok(sheet),
            Err(e) => Err(Status::new("Failed to load sprite sheet: {}", e)),
        }
    }
}

impl SpriteSheetsManager {
    pub fn create(resource_path: &str) -> Self {
        SpriteSheetsManager::new(resource_path, SpriteSheetsLoader {})
    }

    pub fn get_box(&self, key: &str, index: usize) -> Option<Rect> {
        match self.get(key) {
            Some(sheet) => match index < sheet.bounding_boxes.len() {
                true => Some(sheet.bounding_boxes[index]),
                false => None,
            },
            None => None,
        }
    }
}

use serde::{
    de::{SeqAccess, Visitor},
    ser::SerializeSeq,
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
        let mut seq = serializer.serialize_seq(Some(4))?;
        seq.serialize_element(&source.x())?;
        seq.serialize_element(&source.y())?;
        seq.serialize_element(&source.width())?;
        seq.serialize_element(&source.height())?;
        seq.end()
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
