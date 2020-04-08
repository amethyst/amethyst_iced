use crate::backend::IcedRenderer;
use iced_native::widget::container::Renderer;
use iced_native::{Element, Point, Rectangle};

impl Renderer for IcedRenderer {
    type Style = ();

    fn draw<Message>(
        &mut self,
        defaults: &<Self as iced_native::renderer::Renderer>::Defaults,
        _: Rectangle,
        cursor_pos: Point,
        _: &<Self as iced_native::widget::container::Renderer>::Style,
        elem: &Element<'_, Message, Self>,
        layout: iced_native::layout::Layout<'_>,
    ) -> Self::Output {
        elem.draw(self, &defaults, layout, cursor_pos)
    }
}
