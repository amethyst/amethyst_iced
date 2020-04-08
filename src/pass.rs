use amethyst::ecs::{Read, SystemData, World, Write};
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
use amethyst::shrev::{EventChannel, ReaderId};
use amethyst::winit::{Event as WinitEvent, WindowEvent as WinitWindowEvent};
use iced_native::{Cache, Size, UserInterface};

use std::fmt::Debug;

use crate::backend::IcedRenderer;
use crate::sandbox::{Sandbox, SandboxContainer};
use crate::vertex::IcedVertex;

#[derive(Default, Debug)]
pub struct IcedPassDesc<S: Sandbox + Debug> {
    _sandbox: std::marker::PhantomData<S>,
}

impl<B: Backend, S: Sandbox + Debug> RenderGroupDesc<B, World> for IcedPassDesc<S> {
    fn build(
        self,
        _ctx: &GraphContext<B>,
        factory: &mut Factory<B>,
        _queue: QueueId,
        world: &World,
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

        let mut winit_event_channel = Write::<'_, EventChannel<WinitEvent>>::fetch(world);
        let winit_reader_id = winit_event_channel.register_reader();

        Ok(Box::new(IcedPass {
            pipeline,
            pipeline_layout,
            fb_width: framebuffer_width,
            fb_height: framebuffer_height,
            vertex,
            cache: Some(Cache::default()),
            winit_reader_id: Some(winit_reader_id),
            _sandbox: std::marker::PhantomData::<S>,
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
pub struct IcedPass<B: Backend, S: Sandbox + Debug> {
    pipeline: B::GraphicsPipeline,
    pipeline_layout: B::PipelineLayout,
    fb_width: u32,
    fb_height: u32,
    vertex: DynamicVertexBuffer<B, IcedVertex>,
    cache: Option<Cache>,
    winit_reader_id: Option<ReaderId<WinitEvent>>,
    _sandbox: std::marker::PhantomData<S>,
}

impl<B: Backend, S: Sandbox + Debug> IcedPass<B, S> {
    pub fn get_vertex_buffer_mut(&mut self) -> &mut DynamicVertexBuffer<B, IcedVertex> {
        &mut self.vertex
    }

    pub fn get_fb_size(&self) -> (f32, f32) {
        (self.fb_width as f32, self.fb_height as f32)
    }
}

type IcedUpdateSystemData<'a, S> = (
    Read<'a, EventChannel<WinitEvent>>,
    Write<'a, EventChannel<<S as Sandbox>::UIMessage>>,
    Read<'a, SandboxContainer<S>>,
    Write<'a, IcedRenderer>,
);

impl<B: Backend, S: Sandbox + Debug> RenderGroup<B, World> for IcedPass<B, S> {
    fn prepare(
        &mut self,
        factory: &Factory<B>,
        _queue: QueueId,
        index: usize,
        _subpass: hal::pass::Subpass<'_, B>,
        world: &World,
    ) -> PrepareResult {
        let (winit_events, mut ui_messages, sandbox, mut iced_renderer) =
            IcedUpdateSystemData::<'_, S>::fetch(world);
        let reader = self
            .winit_reader_id
            .as_mut()
            .expect("Failed to get ReaderID: IcedUpdateSystem has not been setup.");
        let bounds: Size = [self.fb_width as u16, self.fb_height as u16].into();
        let cache = self.cache.take().unwrap();
        let mut user_interface =
            UserInterface::build(sandbox.view(), bounds, cache, &mut iced_renderer);
        winit_events
            .read(reader)
            .filter_map(|winit_event| match winit_event {
                // TODO: Propper handling of window events, using iced_winit::conversion
                // Possible when Amethyst upgrades to winit 0.22
                WinitEvent::WindowEvent {
                    event: WinitWindowEvent::Resized(size),
                    ..
                } => Some(iced_native::Event::Window(
                    iced_native::window::Event::Resized {
                        width: size.width as u32,
                        height: size.height as u32,
                    },
                )),
                _ => None,
            })
            .flat_map(|iced_event| user_interface.update(vec![iced_event], None, &iced_renderer))
            .for_each(|ui_msg| ui_messages.single_write(ui_msg));
        let primitives = user_interface.draw(&mut iced_renderer);
        primitives.render(self, factory, index);

        self.cache = Some(user_interface.into_cache());
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
            encoder.draw(0..6, 0..2);
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
