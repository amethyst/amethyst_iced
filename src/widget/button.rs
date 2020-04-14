use iced_native::button::Renderer;
use iced_native::{Rectangle, Point, Element, Layout};

use crate::backend::IcedRenderer;
use crate::primitive::AmethystIcedPrimitive;

impl<'a> Renderer for IcedRenderer<'a> {
    const DEFAULT_PADDING: u16 = 5;
    type Style = (); 

    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        bounds: Rectangle,
        cursor_position: Point,
        is_disabled: bool,
        is_pressed: bool,
        _style: &Self::Style,
        content: &Element<'_, Message, Self>,
        content_layout: Layout<'_>,
    ) -> Self::Output {
        // TODO: Make these default colors customizable 
        // + add color to style
        // + handle hovering
        let color = if is_disabled {
            [0.5, 0.5,0.5,1.]
        } else if is_pressed {
            [0.,1.,0.,1.]
        } else {
            [0.,0.,1.,1.] 
        };
        let background = AmethystIcedPrimitive::Quad(bounds, Some(color.into()));
        let children = content.draw(self, defaults, content_layout, cursor_position); 

        AmethystIcedPrimitive::Group(vec![
            background, 
            children
        ])
    }

}