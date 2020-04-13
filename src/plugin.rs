use amethyst::{
    core::ecs::{DispatcherBuilder, World},
    error::Error,
    renderer::{
        bundle::{RenderOrder, RenderPlan, RenderPlugin, Target},
        rendy::{factory::Factory, graph::render::RenderGroupDesc},
        types::Backend,
    },
};

use std::fmt::Debug;

use crate::pass::IcedPassDesc;
use crate::systems::IcedDrawGlyphSystem;

#[derive(Default, Debug)]
pub struct IcedUI;

impl<B: Backend> RenderPlugin<B> for IcedUI {
    fn on_build<'a, 'b>(
        &mut self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(IcedDrawGlyphSystem::<B>::default(), "iced_draw_glyph", &[]);
        Ok(())
    }

    fn on_plan(
        &mut self,
        plan: &mut RenderPlan<B>,
        _factory: &mut Factory<B>,
        _world: &World,
    ) -> Result<(), Error> {
        plan.extend_target(Target::Main, |ctx| {
            // Add our Description
            ctx.add(RenderOrder::Transparent, IcedPassDesc::default().builder())?;
            Ok(())
        });
        Ok(())
    }
}
