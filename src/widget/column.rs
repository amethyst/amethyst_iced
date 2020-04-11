use crate::{backend::IcedRenderer, primitive::AmethystIcedPrimitive};
use iced_native::{column::Renderer as ColumnRenderer, Element, Layout, Point};

impl<'a> ColumnRenderer for IcedRenderer<'a> {
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        content: &[Element<'_, Message, Self>],
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Self::Output {
        AmethystIcedPrimitive::Group(
            content
                .iter()
                .zip(layout.children())
                .map(|(child, layout)| child.draw(self, defaults, layout, cursor_position))
                .collect(),
        )
    }
}
