use amethyst::assets::{Handle, AssetStorage};
use amethyst::renderer::SpriteSheet;
use amethyst::ecs::Read;
use iced_native::renderer::Renderer;

use crate::primitive::AmethystIcedPrimitive;

pub struct IcedRenderer<'a> {
    textures: Read<'a, AssetStorage<SpriteSheet>>,
}

impl<'a> IcedRenderer<'a> {
    pub fn new(textures: Read<'a, AssetStorage<SpriteSheet>>) -> Self {
        IcedRenderer {
            textures,
        }
    }
}

impl<'a> Renderer for IcedRenderer<'a> {
    type Output = AmethystIcedPrimitive;
    type Defaults = ();
}
