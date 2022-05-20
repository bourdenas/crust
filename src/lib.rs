// Declare the modules created from protbufs.
pub mod crust {
    tonic::include_proto!("crust");
}

pub mod action;
pub mod animation;
pub mod components;
pub mod core;
pub mod input;
pub mod resources;
pub mod systems;
