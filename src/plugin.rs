use amethyst::{
    core::ecs::World,
    error::Error,
    renderer::{
        bundle::{RenderOrder, RenderPlan, RenderPlugin, Target},
        rendy::{factory::Factory, graph::render::RenderGroupDesc},
        types::Backend,
    },
};

use std::fmt::Debug;

use crate::pass::IcedPassDesc;

#[derive(Default, Debug)]
pub struct IcedUI; 

impl<B: Backend> RenderPlugin<B> for IcedUI {
    fn on_plan(
        &mut self,
        plan: &mut RenderPlan<B>,
        _factory: &mut Factory<B>,
        _world: &World,
    ) -> Result<(), Error> {
        plan.extend_target(Target::Main, |ctx| {
            // Add our Description
            ctx.add(
                RenderOrder::Transparent,
                IcedPassDesc::default().builder(),
            )?;
            Ok(())
        });
        Ok(())
    }
}
