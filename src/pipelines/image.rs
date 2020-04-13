use amethyst::renderer::{
    batch::OrderedOneLevelBatch,
    pipeline::{PipelineDescBuilder, PipelinesBuilder},
    rendy::{
        command::RenderPassEncoder,
        factory::Factory,
        hal::pso::{self, ShaderStageFlags},
        hal::{self, device::Device},
        mesh::AsVertex,
        shader::{Shader, SpirvShader},
    },
    submodules::{DynamicUniform, DynamicVertexBuffer, TextureId, TextureSub},
    types::Backend,
    util::simple_shader_set,
};
use glam::{Mat4, Vec3};
use glsl_layout::{mat4, AsStd140};

use crate::vertex::image::ImageVertex;

lazy_static::lazy_static! {
     static ref IMAGE_VERTEX: SpirvShader = SpirvShader::from_bytes(
        include_bytes!("../../shaders/compiled/image.vert.spv"),
        ShaderStageFlags::VERTEX,
        "main",
    ).unwrap();

    static ref IMAGE_FRAGMENT: SpirvShader = SpirvShader::from_bytes(
        include_bytes!("../../shaders/compiled/image.frag.spv"),
        ShaderStageFlags::FRAGMENT,
        "main",
    ).unwrap();
}

#[derive(Debug)]
pub struct ImagePipeline<B: Backend> {
    pipeline: B::GraphicsPipeline,
    pipeline_layout: B::PipelineLayout,
    pub textures: TextureSub<B>,
    pub vertex: DynamicVertexBuffer<B, ImageVertex>,
    pub batches: OrderedOneLevelBatch<TextureId, ImageVertex>,
    pub uniforms: DynamicUniform<B, ImageUniform>,
    pub transform: ImageUniform,
}

impl<B: Backend> ImagePipeline<B> {
    pub fn create_pipeline(
        factory: &Factory<B>,
        subpass: hal::pass::Subpass<'_, B>,
        fb_width: u32,
        fb_height: u32,
    ) -> Result<Self, failure::Error> {
        let uniforms =
            DynamicUniform::<B, ImageUniform>::new(factory, pso::ShaderStageFlags::VERTEX)?;
        let textures = TextureSub::new(factory)?;
        let layouts = vec![uniforms.raw_layout(), textures.raw_layout()];
        let pipeline_layout = unsafe {
            factory
                .device()
                .create_pipeline_layout(layouts, None as Option<(_, _)>)
        }?;

        let vertex = DynamicVertexBuffer::<B, ImageVertex>::new();
        let batches = OrderedOneLevelBatch::<TextureId, ImageVertex>::default();

        let shader_vertex = unsafe {
            IMAGE_VERTEX
                .module(factory)
                .expect("Failed to create triangle_vertex module")
        };
        let shader_fragment = unsafe {
            IMAGE_FRAGMENT
                .module(factory)
                .expect("Failed to create triangle_fragment module")
        };

        let pipes = PipelinesBuilder::new()
            .with_pipeline(
                PipelineDescBuilder::new()
                    .with_vertex_desc(&[(ImageVertex::vertex(), pso::VertexInputRate::Vertex)])
                    .with_input_assembler(pso::InputAssemblerDesc::new(
                        hal::Primitive::TriangleList,
                    ))
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

        let fb_width = fb_width as f32;
        let fb_height = fb_height as f32;
        let u_transform =
            Mat4::orthographic_lh(
                -fb_width / 2.,
                fb_width / 2.,
                -fb_height / 2.,
                fb_height / 2.,
                0.1,
                2000.,
            ) * Mat4::from_translation(Vec3::new(0. - fb_width / 2., -fb_height / 2., 0.));
        let u_transform: mat4 = u_transform.to_cols_array_2d().into();
        let transform = ImageUniform { u_transform };

        match pipes {
            Err(e) => {
                unsafe {
                    factory.device().destroy_pipeline_layout(pipeline_layout);
                }
                Err(e)
            }
            Ok(mut pipeline) => {
                let pipeline = pipeline.remove(0);
                Ok(ImagePipeline {
                    pipeline,
                    pipeline_layout,
                    textures,
                    uniforms,
                    vertex,
                    batches,
                    transform,
                })
            }
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
        if self.batches.count() == 0 {
            return;
        }

        encoder.bind_graphics_pipeline(&self.pipeline);
        self.uniforms.bind(index, &self.pipeline_layout, 0, encoder);
        self.vertex.bind(index, 0, 0, encoder);
        self.batches.iter().for_each(|(&tex, verts)| {
            self.textures.bind(&self.pipeline_layout, 1, tex, encoder);
            unsafe {
                encoder.draw(verts, 0..1);
            }
        });
    }
}

#[derive(Clone, Debug, AsStd140)]
#[repr(C, align(4))]
pub struct ImageUniform {
    u_transform: mat4,
}
