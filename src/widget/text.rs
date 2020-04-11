use crate::{backend::IcedRenderer, primitive::AmethystIcedPrimitive};
use iced_native::widget::text::Renderer as TextRenderer;
use iced_native::{Color, Font, HorizontalAlignment, Rectangle, Size, VerticalAlignment};

impl<'a> TextRenderer for IcedRenderer<'a> {
    const DEFAULT_SIZE: u16 = 16;

    fn measure(&self, _content: &str, _size: u16, _font: Font, _bounds: Size) -> (f32, f32) {
        (100., 100.)
    }

    fn draw(
        &mut self,
        _defaults: &Self::Defaults,
        bounds: Rectangle,
        _content: &str,
        _size: u16,
        _font: Font,
        color: Option<Color>,
        _horizontal_alignment: HorizontalAlignment,
        _vertical_alignment: VerticalAlignment,
    ) -> Self::Output {
        AmethystIcedPrimitive::Quad(bounds, color)
    }
}
