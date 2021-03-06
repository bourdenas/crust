use crate::core::Status;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

// A generic resource manager that handles loading external resources and
// maintaining an in-memory cache for quick retrieval.
//
// Simlilar to [resources::ResourceManager] but specific for Resources whose
// lifetime is tied to that of their associated Loader.
pub struct ResourceManagerWithAnnotation<'l, Key, Resource, Loader>
where
    Key: Hash + Eq,
    Loader: 'l + ResourceLoaderWithAnnotation<'l, Resource>,
{
    resource_path: String,
    loader: &'l Loader,
    cache: HashMap<Key, Rc<Resource>>,
}

impl<'l, Key, Resource, Loader> ResourceManagerWithAnnotation<'l, Key, Resource, Loader>
where
    Key: Hash + Eq,
    Loader: ResourceLoaderWithAnnotation<'l, Resource>,
{
    pub fn new(resource_path: &str, loader: &'l Loader) -> Self {
        ResourceManagerWithAnnotation {
            resource_path: resource_path.to_owned(),
            loader,
            cache: HashMap::new(),
        }
    }

    // Generics magic to allow a HashMap to use String as a key
    // while allowing it to use &str for gets
    pub fn load<D>(&mut self, details: &D) -> Result<Rc<Resource>, Status>
    where
        Loader: ResourceLoaderWithAnnotation<'l, Resource, Args = D>,
        D: Eq + Hash + ?Sized,
        Key: Borrow<D> + for<'a> From<&'a D>,
    {
        self.cache.get(details).cloned().map_or_else(
            || {
                let resource = Rc::new(self.loader.load(&self.resource_path, details)?);
                self.cache.insert(details.into(), resource.clone());
                Ok(resource)
            },
            Ok,
        )
    }

    pub fn get<D>(&self, details: &D) -> Option<Rc<Resource>>
    where
        D: Eq + Hash + ?Sized,
        Key: Borrow<D> + for<'d> From<&'d D>,
    {
        match self.cache.get(details) {
            Some(resource) => Some(Rc::clone(resource)),
            None => None,
        }
    }
}

/// Generic trait to load resource.
pub trait ResourceLoaderWithAnnotation<'l, Resource> {
    type Args: ?Sized;

    fn load(&'l self, path: &str, resource: &Self::Args) -> Result<Resource, Status>;
}
