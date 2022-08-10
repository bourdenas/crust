use super::{ResourceLoader, ResourceManager};
use crate::core::Status;
use sdl2::rect::Rect;
use specs::BitSet;

pub type SpriteManager = ResourceManager<String, Sprite, SpriteLoader>;

impl SpriteManager {
    pub fn get_collision_mask(&self, texture_id: &str, frame_index: usize) -> Option<&BitSet> {
        match self.get(texture_id) {
            Some(sprite) => sprite.frames[frame_index].bitmask.as_ref(),
            None => None,
        }
    }
}

pub struct SpriteLoader;

impl ResourceLoader<Sprite> for SpriteLoader {
    type Args = str;

    fn load(&self, path: &str, resource: &str) -> Result<Sprite, Status> {
        let filename = format!("{path}/{resource}.json");
        let json = std::fs::read(&filename).expect(&format!("Failed to read '{filename}'"));
        match serde_json::from_slice::<Sprite>(&json) {
            Ok(sheet) => Ok(sheet),
            Err(e) => Err(Status::new("Failed to parse sprite sheet: {}", e)),
        }
    }
}

impl SpriteManager {
    pub fn create(resource_path: &str) -> Self {
        SpriteManager::new(resource_path, SpriteLoader {})
    }

    pub fn get_box(&self, key: &str, index: usize) -> Option<Rect> {
        match self.get(key) {
            Some(sheet) => match index < sheet.frames.len() {
                true => Some(sheet.frames[index].bounding_box),
                false => None,
            },
            None => None,
        }
    }
}

#[cfg(test)]
impl SpriteManager {
    pub fn mock(sprites: Vec<Sprite>) -> Self {
        let mut mgr = SpriteManager::new("", SpriteLoader {});
        mgr.set_resources(
            sprites
                .into_iter()
                .map(|sprite| (sprite.texture_id.clone(), sprite))
                .collect(),
        );
        mgr
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
pub struct Sprite {
    pub texture_id: String,
    pub frames: Vec<Frame>,
}

#[serde_with::serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    #[serde_as(as = "CrustRect")]
    pub bounding_box: Rect,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<BitMask>")]
    pub bitmask: Option<BitSet>,
}

struct BitMask;

impl serde_with::SerializeAs<BitSet> for BitMask {
    fn serialize_as<S>(source: &BitSet, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;
        for bit in source {
            seq.serialize_element(&bit)?;
        }
        seq.end()
    }
}

impl<'de> serde_with::DeserializeAs<'de, BitSet> for BitMask {
    fn deserialize_as<D>(deserializer: D) -> Result<BitSet, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BitMaskVisitor;

        impl<'de> Visitor<'de> for BitMaskVisitor {
            type Value = BitSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a json vector containing positive integers")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut bitset = BitSet::new();
                'sequence: loop {
                    match seq.next_element()? {
                        Some(value) => {
                            bitset.add(value);
                        }
                        None => break 'sequence,
                    }
                }
                Ok(bitset)
            }
        }

        deserializer.deserialize_seq(BitMaskVisitor {})
    }
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
