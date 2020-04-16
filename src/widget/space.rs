use iced_native::{space::Renderer, Rectangle};

use crate::backend::IcedRenderer;
use crate::primitive::AmethystIcedPrimitive;

impl<'a> Renderer for IcedRenderer<'a> {
    fn draw(&mut self, _bounds: Rectangle) -> Self::Output {
        AmethystIcedPrimitive::None
    }
}
