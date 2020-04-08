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
use crate::sandbox::Sandbox;

#[derive(Default, Debug)]
pub struct IcedUI<S: Sandbox + Debug> {
    _sandbox: std::marker::PhantomData<S>,
}

impl<B: Backend, S: Sandbox + Debug> RenderPlugin<B> for IcedUI<S> {
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
                IcedPassDesc::<S>::default().builder(),
            )?;
            Ok(())
        });
        Ok(())
    }
}
