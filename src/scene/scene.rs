use sdl2::rect::Rect;

pub struct Scene {
    pub layers: Vec<SceneLayer>,
    pub bounds: Rect,
}
#[derive(Default)]
pub struct SceneLayer {
    pub tiles: Vec<TileInfo>,
}

pub struct TileInfo {
    pub texture_id: String,
    pub texture_position: Rect,
    pub canvas_position: Rect,
}
