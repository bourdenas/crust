use crate::core::Status;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

// A generic resource manager that handles loading external resources and
// maintaining an in-memory cache for quick retrieval.
pub struct ResourceManager<Key, Resource, Loader>
where
    Key: Hash + Eq,
    Loader: ResourceLoader<Resource>,
{
    resource_path: String,
    loader: Loader,
    cache: HashMap<Key, Resource>,
}

impl<Key, Resource, Loader> ResourceManager<Key, Resource, Loader>
where
    Key: Hash + Eq,
    Loader: ResourceLoader<Resource>,
{
    pub fn new(resource_path: &str, loader: Loader) -> Self {
        ResourceManager {
            resource_path: resource_path.to_owned(),
            loader,
            cache: HashMap::new(),
        }
    }

    // Generics magic to allow a HashMap to use String as a key
    // while allowing it to use &str for gets
    pub fn load<'a, D>(&'a mut self, details: &D) -> Result<&'a Resource, Status>
    where
        Loader: ResourceLoader<Resource, Args = D>,
        D: Eq + Hash + ?Sized,
        Key: Borrow<D> + for<'d> From<&'d D>,
    {
        if let None = self.cache.get(details) {
            let resource = self.loader.load(&self.resource_path, details)?;
            self.cache.insert(details.into(), resource);
        }

        match self.cache.get(details) {
            Some(resource) => Ok(resource),
            None => Err(Status::not_found("resource not found")),
        }
    }

    pub fn get<'a, D>(&'a self, details: &D) -> Option<&'a Resource>
    where
        D: Eq + Hash + ?Sized,
        Key: Borrow<D> + for<'d> From<&'d D>,
    {
        self.cache.get(details)
    }
}

/// Generic trait to load resource.
pub trait ResourceLoader<Resource> {
    type Args: ?Sized;

    fn load(&self, path: &str, resource: &Self::Args) -> Result<Resource, Status>;
}
