use iced_native::renderer::Renderer;

use crate::primitive::AmethystIcedPrimitive;

#[derive(Debug, Default)]
pub struct IcedRenderer;

impl Renderer for IcedRenderer {
    type Output = AmethystIcedPrimitive;
    type Defaults = ();
}
