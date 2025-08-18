use crate::{drawer_helper::Position, loading_data::LoadingData, loading_drawer::{draw_loader, draw_print_history, LoadingDrawer}, loading_handler::LoadingHandler};


#[derive(Copy, Clone)]
pub struct DrawableElement {
    pub pos: Position,
    _draw: fn(&Self, &mut LoadingDrawer, &mut LoadingData)
}
impl DrawableElement {
    pub fn draw(&self, drawer: &mut LoadingDrawer, data: &mut LoadingData) {
        (self._draw)(self, drawer, data);
    }
}
#[derive(Copy, Clone)]
pub struct DrawableElementFill {
    _draw: fn(&Self, &mut LoadingDrawer, &mut LoadingData)
}
impl DrawableElementFill {
    pub fn draw(&self, drawer: &mut LoadingDrawer, data: &mut LoadingData) {
        (self._draw)(self, drawer, data);
    }
}

pub static LOADING_BAR: DrawableElement = DrawableElement { pos: Position::TOP, _draw: draw_loader };

//pub static DRAW_INPUT_FIELD: DrawableElement = DrawableElement { pos: Position::BOTTOM, draw: NOT_IMPLEMENTED };

pub static DRAW_PRINT_HISTORY: DrawableElementFill = DrawableElementFill { _draw: draw_print_history };

pub struct DrawOrdering {
    pub elements: Vec<DrawableElement>,
    pub fill_element: DrawableElementFill
}