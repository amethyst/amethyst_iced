use iced_native::pane_grid::Renderer;
use iced_native::{
    pane_grid::{Axis, Pane},
    Element, Layout, Point,
};

use crate::backend::IcedRenderer;
use crate::primitive::AmethystIcedPrimitive;

impl<'a> Renderer for IcedRenderer<'a> {
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        content: &[(Pane, Element<'_, Message, Self>)],
        dragging: Option<Pane>,
        _resizing: Option<Axis>,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Self::Output {
        let pane_cursor_position = if dragging.is_some() {
            // TODO: Remove once cursor availability is encoded in the type
            // system
            Point::new(-1.0, -1.0)
        } else {
            cursor_position
        };

        AmethystIcedPrimitive::Group(
            content
                .iter()
                .zip(layout.children())
                .enumerate()
                .map(|(_i, ((_id, pane), layout))| {
                    pane.draw(self, defaults, layout, pane_cursor_position)
                })
                .collect(),
        )
    }
}
