use amethyst::ecs::{SystemData, World, Write};
use amethyst::renderer::{
    pipeline::{PipelineDescBuilder, PipelinesBuilder},
    rendy::{
        command::{QueueId, RenderPassEncoder},
        factory::Factory,
        graph::{
            render::{PrepareResult, RenderGroup, RenderGroupDesc},
            GraphContext, NodeBuffer, NodeImage,
        },
        hal::pso::{self, ShaderStageFlags},
        hal::{self, device::Device},
        mesh::AsVertex,
        shader::{Shader, SpirvShader},
    },
    submodules::{DynamicUniform, DynamicVertexBuffer},
    types::Backend,
    util::simple_shader_set,
};

use crate::{primitive::IcedPrimitives, vertex::IcedVertex};

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
        let env = DynamicUniform::<B, IcedVertex>::new(factory, pso::ShaderStageFlags::VERTEX)?;
        let vertex = DynamicVertexBuffer::<B, IcedVertex>::new();

        let (pipeline, pipeline_layout) = build_pipeline(
            factory,
            subpass,
            framebuffer_width,
            framebuffer_height,
            vec![env.raw_layout()],
        )?;

        Ok(Box::new(IcedPass {
            pipeline,
            pipeline_layout,
            vertex,
        }))
    }
}

lazy_static::lazy_static! {
     static ref CUSTOM_VERTEX: SpirvShader = SpirvShader::from_bytes(
        include_bytes!("../shaders/compiled/triangle.vert.spv"),
        ShaderStageFlags::VERTEX,
        "main",
    ).unwrap();

    static ref CUSTOM_FRAGMENT: SpirvShader = SpirvShader::from_bytes(
        include_bytes!("../shaders/compiled/triangle.frag.spv"),
        ShaderStageFlags::FRAGMENT,
        "main",
    ).unwrap();

}

fn build_pipeline<B: Backend>(
    factory: &Factory<B>,
    subpass: hal::pass::Subpass<'_, B>,
    fb_width: u32,
    fb_height: u32,
    layouts: Vec<&B::DescriptorSetLayout>,
) -> Result<(B::GraphicsPipeline, B::PipelineLayout), failure::Error> {
    let pipeline_layout = unsafe {
        factory
            .device()
            .create_pipeline_layout(layouts, None as Option<(_, _)>)
    }?;

    let shader_vertex = unsafe {
        CUSTOM_VERTEX.module(factory).expect(
            "Failed to create shader_vertex
module",
        )
    };
    let shader_fragment = unsafe {
        CUSTOM_FRAGMENT.module(factory).expect(
            "Failed to create shader_fra
gment module",
        )
    };

    let pipes = PipelinesBuilder::new()
        .with_pipeline(
            PipelineDescBuilder::new()
                .with_vertex_desc(&[(IcedVertex::vertex(), pso::VertexInputRate::Vertex)])
                .with_input_assembler(pso::InputAssemblerDesc::new(hal::Primitive::TriangleList))
                .with_shaders(simple_shader_set(&shader_vertex, Some(&shader_fragment)))
                .with_layout(&pipeline_layout)
                .with_subpass(subpass)
                .with_framebuffer_size(fb_width, fb_height)
                .with_blend_targets(vec![pso::ColorBlendDesc {
                    mask: pso::ColorMask::ALL,
                    blend: Some(pso::BlendState::ALPHA),
                }]),
        )
        .build(factory, None);

    unsafe {
        factory.destroy_shader_module(shader_vertex);
        factory.destroy_shader_module(shader_fragment);
    }

    match pipes {
        Err(e) => {
            unsafe {
                factory.device().destroy_pipeline_layout(pipeline_layout);
            }
            Err(e)
        }
        Ok(mut pipes) => Ok((pipes.remove(0), pipeline_layout)),
    }
}

#[derive(Debug)]
pub struct IcedPass<B: Backend> {
    pipeline: B::GraphicsPipeline,
    pipeline_layout: B::PipelineLayout,
    vertex: DynamicVertexBuffer<B, IcedVertex>,
}

impl<B: Backend> IcedPass<B> {
    pub fn get_vertex_buffer_mut(&mut self) -> &mut DynamicVertexBuffer<B, IcedVertex> {
        &mut self.vertex
    }
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
        if let Some(iced_primitives ) = iced_primitives.0.take() {
            iced_primitives.render(self, factory, index);
        }
        PrepareResult::DrawRecord
    }

    fn draw_inline(
        &mut self,
        mut encoder: RenderPassEncoder<'_, B>,
        index: usize,
        _subpass: hal::pass::Subpass<'_, B>,
        _aux: &World,
    ) {
        encoder.bind_graphics_pipeline(&self.pipeline);
        self.vertex.bind(index, 0, 0, &mut encoder);
        unsafe {
            encoder.draw(0..6, 0..1);
            encoder.draw(0..6, 0..1);
        }
    }

    fn dispose(self: Box<Self>, factory: &mut Factory<B>, _aux: &World) {
        unsafe {
            factory.device().destroy_graphics_pipeline(self.pipeline);
            factory
                .device()
                .destroy_pipeline_layout(self.pipeline_layout);
        }
    }
}
