use amethyst::ecs::{World, SystemData, Read};
use amethyst::renderer::{
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

use crate::vertex::TextVertex;
use crate::systems::{TextVertexContainer, GlyphAtlas};

lazy_static::lazy_static! {
     static ref TEXT_VERTEX: SpirvShader = SpirvShader::from_bytes(
        include_bytes!("../../shaders/compiled/text.vert.spv"),
        ShaderStageFlags::VERTEX,
        "main",
    ).unwrap();

    static ref TEXT_FRAGMENT: SpirvShader = SpirvShader::from_bytes(
        include_bytes!("../../shaders/compiled/text.frag.spv"),
        ShaderStageFlags::FRAGMENT,
        "main",
    ).unwrap();
}

#[derive(Debug)]
pub struct TextPipeline<B: Backend> {
    pipeline: B::GraphicsPipeline,
    pipeline_layout: B::PipelineLayout,
    textures: TextureSub<B>,
    pub vertex: DynamicVertexBuffer<B, TextVertex>,
    pub uniforms: DynamicUniform<B, TextUniform>,
    pub transform: TextUniform,
    glyph_atlas_id: Option<TextureId>,
}

impl<B: Backend> TextPipeline<B> {
    pub fn create_pipeline(
        factory: &Factory<B>,
        subpass: hal::pass::Subpass<'_, B>,
        fb_width: u32,
        fb_height: u32,
    ) -> Result<Self, failure::Error> {
        let uniforms =
            DynamicUniform::<B, TextUniform>::new(factory, pso::ShaderStageFlags::VERTEX)?;
        let textures = TextureSub::new(factory)?;
        let layouts = vec![uniforms.raw_layout(), textures.raw_layout()];
        let pipeline_layout = unsafe {
            factory
                .device()
                .create_pipeline_layout(layouts, None as Option<(_, _)>)
        }?;

        let vertex = DynamicVertexBuffer::<B, TextVertex>::new();

        let shader_vertex = unsafe {
            TEXT_VERTEX
                .module(factory)
                .expect("Failed to create triangle_vertex module")
        };
        let shader_fragment = unsafe {
            TEXT_FRAGMENT
                .module(factory)
                .expect("Failed to create triangle_fragment module")
        };

        let pipes = PipelinesBuilder::new()
            .with_pipeline(
                PipelineDescBuilder::new()
                    .with_vertex_desc(&[(TextVertex::vertex(), pso::VertexInputRate::Vertex)])
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
        let transform = TextUniform { u_transform };

        match pipes {
            Err(e) => {
                unsafe {
                    factory.device().destroy_pipeline_layout(pipeline_layout);
                }
                Err(e)
            }
            Ok(mut pipeline) => {
                let pipeline = pipeline.remove(0);
                Ok(TextPipeline {
                    pipeline,
                    pipeline_layout,
                    textures,
                    uniforms,
                    vertex,
                    transform,
                    glyph_atlas_id: None,
                })
            }
        }
    }

    pub fn bind_texture_id(&mut self, factory: &Factory<B>, world: &World) {
        if self.glyph_atlas_id.is_some() {
            return;
        }
        let glyph_atlas = Read::<'_, GlyphAtlas>::fetch(world);    
        let tex_handle = (*glyph_atlas).0.as_ref().unwrap();
        let tex_id = self.textures.insert(factory, world, tex_handle, hal::image::Layout::General).unwrap().0;
        self.glyph_atlas_id = Some(tex_id);
    }

    
    pub fn draw(&self, encoder: &mut RenderPassEncoder<'_, B>, index: usize, world: &World) {
        if self.glyph_atlas_id.is_none() {
            return;
        }
        let tex_id = self.glyph_atlas_id.unwrap();
        
        let text_vertex_container = Read::<'_, TextVertexContainer>::fetch(world);
        if text_vertex_container.0.len() == 0 {
            return; 
        }

        encoder.bind_graphics_pipeline(&self.pipeline);
        self.uniforms.bind(index, &self.pipeline_layout, 0, encoder);
        self.vertex.bind(index, 0, 0, encoder);
        self.textures.bind(&self.pipeline_layout, 1, tex_id, encoder);
        unsafe {
            encoder.draw(0..text_vertex_container.0.len() as u32, 0..1);
            //encoder.draw(0..6, 0..1);
        }
    }
}

#[derive(Clone, Debug, AsStd140)]
#[repr(C, align(4))]
pub struct TextUniform {
    u_transform: mat4,
}
