use amethyst::{
    core::SystemBundle,
    ecs::{DispatcherBuilder, World},
    shrev::EventChannel,
    Error,
};
use glyph_brush::GlyphBrushBuilder;

use crate::{
    sandbox::Sandbox,
    primitive::IcedPrimitives,
    systems::{IcedDrawSystem, IcedInteropSystem},
    IcedGlyphBrush
};

pub struct IcedBundle<S: Sandbox> {
    _sandbox: std::marker::PhantomData<S>,
}

impl<S: Sandbox> Default for IcedBundle<S> {
    fn default() -> Self {
        IcedBundle::new()
    }
}

impl<S: Sandbox> IcedBundle<S> {
    /// Creates a new IcedBundle containing a Sandboxed application
    pub fn new() -> Self {
        IcedBundle { _sandbox: std::marker::PhantomData }
    }
}

impl<'a, 'b, S: Sandbox> SystemBundle<'a, 'b> for IcedBundle<S> {
    fn build(
        self,
        world: &mut World,
        dispatcher: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        // Creates communication channels for the Sandbox
        world.insert(EventChannel::<S::UIMessage>::default());
        world.insert(EventChannel::<S::GameMessage>::default());
        world.insert(IcedPrimitives::default());
        let square_ttf: &[u8] = include_bytes!("../font/square.ttf");
        world.insert::<IcedGlyphBrush>(GlyphBrushBuilder::using_font_bytes(square_ttf).build());

        // Adds Iced-related systems
        dispatcher.add(IcedInteropSystem::<S>::default(), "iced_interop", &[]);
        dispatcher.add(
            IcedDrawSystem::<S>::default(),
            "iced_draw",
            &["iced_interop"],
        );

        Ok(())
    }
}
