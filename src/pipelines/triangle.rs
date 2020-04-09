use amethyst::renderer::{
    pipeline::{PipelineDescBuilder, PipelinesBuilder},
    rendy::{
        command::{RenderPassEncoder},
        factory::Factory,
        hal::pso::{self, ShaderStageFlags},
        hal::{self, device::Device},
        mesh::AsVertex,
        shader::{Shader, SpirvShader},
    },
    submodules::{DynamicUniform, DynamicVertexBuffer, DynamicIndexBuffer},
    types::Backend,
    util::simple_shader_set,
};
use glsl_layout::{mat4, AsStd140};

use crate::vertex::TriangleVertex;

lazy_static::lazy_static! {
     static ref TRIANGLE_VERTEX: SpirvShader = SpirvShader::from_bytes(
        include_bytes!("../../shaders/compiled/triangle.vert.spv"),
        ShaderStageFlags::VERTEX,
        "main",
    ).unwrap();

    static ref TRIANGLE_FRAGMENT: SpirvShader = SpirvShader::from_bytes(
        include_bytes!("../../shaders/compiled/triangle.frag.spv"),
        ShaderStageFlags::FRAGMENT,
        "main",
    ).unwrap();
}

#[derive(Debug)]
pub struct TrianglePipeline<B: Backend> {
    pipeline: B::GraphicsPipeline,
    pipeline_layout: B::PipelineLayout,
    pub vertex: DynamicVertexBuffer<B, TriangleVertex>,
    pub indices: DynamicIndexBuffer<B, usize>,
    pub uniforms: DynamicUniform<B, TriangleUniform>
}

impl<B: Backend> TrianglePipeline<B> {
    pub fn create_pipeline(
        factory: &Factory<B>,
        subpass: hal::pass::Subpass<'_, B>,
        fb_width: u32,
        fb_height: u32,
    ) -> Result<Self, failure::Error> {
        let uniforms = DynamicUniform::<B, TriangleUniform>::new(factory, pso::ShaderStageFlags::VERTEX)?;
        let layouts = vec![uniforms.raw_layout()];
        let pipeline_layout = unsafe {
            factory
                .device()
                .create_pipeline_layout(layouts, None as Option<(_, _)>)
        }?;

        let vertex = DynamicVertexBuffer::<B, TriangleVertex>::new();
        let indices = DynamicIndexBuffer::<B, usize>::new();

        let shader_vertex = unsafe {
            TRIANGLE_VERTEX.module(factory).expect(
                "Failed to create shader_vertex
    module",
            )
        };
        let shader_fragment = unsafe {
            TRIANGLE_FRAGMENT.module(factory).expect(
                "Failed to create shader_fra
    gment module",
            )
        };

        let pipes = PipelinesBuilder::new()
            .with_pipeline(
                PipelineDescBuilder::new()
                    .with_vertex_desc(&[(TriangleVertex::vertex(), pso::VertexInputRate::Vertex)])
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
            Ok(mut pipeline) => {
                let pipeline = pipeline.remove(0); 
                Ok( 
                    TrianglePipeline {
                        pipeline,
                        pipeline_layout,
                        uniforms, 
                        vertex,
                        indices,
                    }
                )
            },
        }
    }

    pub fn dispose(self, factory: &mut Factory<B>) {
       unsafe {
            factory.device().destroy_graphics_pipeline(self.pipeline);
            factory
                .device()
                .destroy_pipeline_layout(self.pipeline_layout);
        } 
    }

    pub fn draw(&self, encoder: &mut RenderPassEncoder<'_, B>, index: usize) {
        encoder.bind_graphics_pipeline(&self.pipeline);
        self.vertex.bind(index, 0, 0, encoder);
        unsafe {
            encoder.draw(0..6, 0..1);
        }
    }
}

#[derive(Clone, Debug, AsStd140)]
#[repr(C)]
pub struct TriangleUniform {
    transform: mat4, 
}
