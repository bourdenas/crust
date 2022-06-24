use super::{ResourceLoaderWithAnnotation, ResourceManagerWithAnnotation};
use crate::core::Status;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};

pub type TextureManager<'l, T> =
    ResourceManagerWithAnnotation<'l, String, Texture<'l>, TextureCreator<T>>;

impl<'l, T> ResourceLoaderWithAnnotation<'l, Texture<'l>> for TextureCreator<T> {
    type Args = str;

    fn load(&'l self, path: &str, resource: &str) -> Result<Texture, Status> {
        println!("Loading '{}/{}.png'", path, resource);
        Ok(self.load_texture(format!("{}/{}.png", path, resource))?)
    }
}
