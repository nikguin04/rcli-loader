use crate::{drawing::{drawer_helper::Position, loading_drawer::LoadingDrawer}, structure::loading_data::LoadingData};


#[derive(Copy, Clone)]
pub struct DrawableElement {
    pub pos: Position,
    pub draw: fn(&LoadingDrawer, &Self, &mut LoadingData, offset: usize) -> usize
}
#[derive(Copy, Clone)]
pub struct DrawableElementFill {
    pub draw: fn(&LoadingDrawer, &Self, &mut LoadingData, offset: usize, remaining_height: usize) -> usize
}

pub static LOADING_BAR: DrawableElement = DrawableElement { pos: Position::TOP, draw: LoadingDrawer::draw_loader };

pub static DRAW_INPUT_FIELD: DrawableElement = DrawableElement { pos: Position::BOTTOM, draw: LoadingDrawer::draw_input_field };

pub static DRAW_PRINT_HISTORY: DrawableElementFill = DrawableElementFill { draw: LoadingDrawer::draw_print_history };

pub struct DrawOrdering {
    pub elements: Vec<DrawableElement>,
    pub fill_element: DrawableElementFill
}