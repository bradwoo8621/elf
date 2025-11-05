use super::BaseDataModel;

pub trait GraphicPosition: BaseDataModel {
    fn x(&self) -> Option<f32>;
    fn y(&self) -> Option<f32>;
}

pub trait GraphicSize: BaseDataModel {
    fn width(&self) -> Option<f32>;
    fn height(&self) -> Option<f32>;
}

pub struct GraphicRect {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

impl BaseDataModel for GraphicRect {}

impl GraphicPosition for GraphicRect {
    fn x(&self) -> Option<f32> {
        self.x
    }

    fn y(&self) -> Option<f32> {
        self.y
    }
}

impl GraphicSize for GraphicRect {
    fn width(&self) -> Option<f32> {
        self.width
    }

    fn height(&self) -> Option<f32> {
        self.height
    }
}
