use crate::{backend::IcedRenderer, primitive::AmethystIcedPrimitive};
use glyph_brush::{rusttype::Scale, GlyphCruncher, Section};
use iced_native::widget::text::Renderer as TextRenderer;
use iced_native::{Color, Font, HorizontalAlignment, Rectangle, Size, VerticalAlignment};

impl<'a> TextRenderer for IcedRenderer<'a> {
    const DEFAULT_SIZE: u16 = 16;

    fn measure(&self, content: &str, size: u16, _font: Font, bounds: Size) -> (f32, f32) {
        if let Some(measurement) = self.glyph_brush.borrow_mut().glyph_bounds(Section {
            text: content,
            scale: Scale::uniform(size as f32),
            bounds: (bounds.width, bounds.height),
            ..Default::default()
        }) {
            (measurement.width(), measurement.height())
        } else {
            (100., 100.)
        }
    }

    fn draw(
        &mut self,
        _defaults: &Self::Defaults,
        bounds: Rectangle,
        content: &str,
        size: u16,
        _font: Font,
        color: Option<Color>,
        horizontal_alignment: HorizontalAlignment,
        _vertical_alignment: VerticalAlignment,
    ) -> Self::Output {
        let color = color.unwrap_or(Color::WHITE);
        let color = [color.r, color.g, color.b, color.a];
        AmethystIcedPrimitive::Text {
            bounds,
            content: content.to_string(),
            size,
            color,
            horizontal_alignment,
        }
    }
}
