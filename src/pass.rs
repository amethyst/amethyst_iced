use amethyst::ecs::{SystemData, World, Write};
use amethyst::renderer::{
    rendy::{
        command::{QueueId, RenderPassEncoder},
        factory::Factory,
        graph::{
            render::{PrepareResult, RenderGroup, RenderGroupDesc},
            GraphContext, NodeBuffer, NodeImage,
        },
        hal::{self},
    },
    types::Backend,
};
use glsl_layout::AsStd140;

use crate::pipelines::{ImagePipeline, TrianglePipeline};
use crate::{vertex::TriangleVertex, primitive::IcedPrimitives};

#[derive(Default, Debug)]
pub struct IcedPassDesc;

impl<B: Backend> RenderGroupDesc<B, World> for IcedPassDesc {
    fn build(
        self,
        _ctx: &GraphContext<B>,
        factory: &mut Factory<B>,
        _queue: QueueId,
        _world: &World,
        framebuffer_width: u32,
        framebuffer_height: u32,
        subpass: hal::pass::Subpass<'_, B>,
        _buffers: Vec<NodeBuffer>,
        _images: Vec<NodeImage>,
    ) -> Result<Box<dyn RenderGroup<B, World>>, failure::Error> {
        let triangle_pipeline = TrianglePipeline::create_pipeline(
            factory,
            subpass,
            framebuffer_width,
            framebuffer_height,
        )?;

        let image_pipeline = ImagePipeline::create_pipeline(
            factory,
            subpass, 
            framebuffer_width,
            framebuffer_height
        )?;

        Ok(Box::new(IcedPass { triangle_pipeline, image_pipeline }))
    }
}

#[derive(Debug)]
pub struct IcedPass<B: Backend> {
    pub triangle_pipeline: TrianglePipeline<B>,
    pub image_pipeline: ImagePipeline<B>,
}

impl<B: Backend> RenderGroup<B, World> for IcedPass<B> {
    fn prepare(
        &mut self,
        factory: &Factory<B>,
        _queue: QueueId,
        index: usize,
        _subpass: hal::pass::Subpass<'_, B>,
        world: &World,
    ) -> PrepareResult {
        let mut iced_primitives = Write::<'_, IcedPrimitives>::fetch(world);
        self.triangle_pipeline.vertices = vec![];
        self.triangle_pipeline.uniforms.write(factory, index, self.triangle_pipeline.transform.std140());
        self.image_pipeline.batches.swap_clear();
        self.image_pipeline.uniforms.write(factory, index, self.image_pipeline.transform.std140());
        if let Some(iced_primitives) = iced_primitives.0.take() {
            iced_primitives.render(self, factory, index, world);
        }

        self.image_pipeline.vertex.write(factory, index, 6, Some(self.image_pipeline.batches.data()));
        self.triangle_pipeline.vertex.write(factory, index, self.triangle_pipeline.vertices.len() as u64, Some(self.triangle_pipeline.vertices.clone().into_iter().collect::<Box<[TriangleVertex]>>())); 
        PrepareResult::DrawRecord
    }

    fn draw_inline(
        &mut self,
        mut encoder: RenderPassEncoder<'_, B>,
        index: usize,
        _subpass: hal::pass::Subpass<'_, B>,
        _aux: &World,
    ) {
        self.triangle_pipeline.draw(&mut encoder, index);
        self.image_pipeline.draw(&mut encoder, index);
    }

    fn dispose(self: Box<Self>, factory: &mut Factory<B>, _aux: &World) {
        self.triangle_pipeline.dispose(factory);
        self.image_pipeline.dispose(factory);
    }
}
