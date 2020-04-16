use iced_native::progress_bar::Renderer as Renderer;
use iced_native::Rectangle;

use crate::backend::IcedRenderer;
use crate::primitive::AmethystIcedPrimitive;

impl<'a> Renderer for IcedRenderer<'a> {
    type Style = ();

    const DEFAULT_HEIGHT: u16 = 30;

    fn draw(
        &self,
        bounds: Rectangle,
        range: std::ops::RangeInclusive<f32>,
        value: f32,
        _style_sheet: &Self::Style,
    ) -> Self::Output {
        let (range_start, range_end) = range.into_inner();
        let active_progress_width = bounds.width
            * ((value - range_start) / (range_end - range_start).max(1.0));

        let background = AmethystIcedPrimitive::Quad(
            bounds,
            Some([1.,1.,1.,1.].into()),
        );

        if active_progress_width > 0.0 {
            let bar = AmethystIcedPrimitive::Quad(
                Rectangle {
                    width: active_progress_width,
                    ..bounds
                },
                Some([0.,1.,0.,1.].into())
            );

            AmethystIcedPrimitive::Group(
                vec![background, bar],
            )
        } else {
            background
        }
    }
}