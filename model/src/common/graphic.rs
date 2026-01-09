use crate::{BaseDataModel, Storable};
use elf_model_marco::adapt_model;

#[adapt_model(storable)]
pub struct GraphicPosition {
    pub x: Option<f32>,
    pub y: Option<f32>,
}

#[adapt_model(storable)]
pub struct GraphicSize {
    pub width: Option<f32>,
    pub height: Option<f32>,
}

#[adapt_model(storable)]
pub struct GraphicRect {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
}
