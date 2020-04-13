use amethyst::assets::{AssetStorage, Handle};
use amethyst::ecs::{Read, System, Write, WriteExpect};
use amethyst::renderer::{
    rendy::{
        command::QueueId,
        factory::{Factory, ImageState},
        hal,
        texture::{pixel::R8Unorm, TextureBuilder},
    },
    types::Backend,
    Texture,
};
use glyph_brush::{BrushAction, BrushError};

use crate::vertex::TextVertex;
use crate::IcedGlyphBrush;

pub struct IcedDrawGlyphSystem<B: Backend> {
    _backend: std::marker::PhantomData<B>,
}

impl<B: Backend> Default for IcedDrawGlyphSystem<B> {
    fn default() -> Self {
        IcedDrawGlyphSystem {
            _backend: std::marker::PhantomData,
        }
    }
}

#[derive(Default)]
pub struct GlyphAtlas(pub Option<Handle<Texture>>);

#[derive(Default)]
pub struct TextVertexContainer(pub Vec<TextVertex>);

impl<'a, B: Backend> System<'a> for IcedDrawGlyphSystem<B> {
    type SystemData = (
        WriteExpect<'a, IcedGlyphBrush>,
        Write<'a, AssetStorage<Texture>>,
        WriteExpect<'a, Factory<B>>,
        Option<Read<'a, QueueId>>,
        Write<'a, GlyphAtlas>,
        Write<'a, TextVertexContainer>,
    );

    fn run(
        &mut self,
        (
            mut iced_glyph_brush,
            mut asset_textures,
            mut factory,
            queue,
            mut glyph_atlas,
            mut text_vertex_container,
        ): Self::SystemData,
    ) {
        if queue.is_none() {
            return;
        }
        let queue = queue.unwrap();
        let queue = *queue;
        let glyph_atlas = glyph_atlas.0.get_or_insert_with(|| {
            let (w, h) = iced_glyph_brush.texture_dimensions();
            asset_textures.insert(create_glyph_texture(&mut *factory, queue, w, h))
        });

        let glyph_tex = asset_textures
            .get(glyph_atlas)
            .and_then(B::unwrap_texture)
            .unwrap();
        let action = iced_glyph_brush.process_queued(
            |rect, data| { 
                println!("uploading a texture, size {:?}", rect); 
                unsafe {
                factory
                    .upload_image(
                        glyph_tex.image().clone(),
                        rect.width(),
                        rect.height(),
                        hal::image::SubresourceLayers {
                            aspects: hal::format::Aspects::COLOR,
                            level: 0,
                            layers: 0..1,
                        },
                        hal::image::Offset {
                            x: rect.min.x as _,
                            y: rect.min.y as _,
                            z: 0,
                        },
                        hal::image::Extent {
                            width: rect.width(),
                            height: rect.height(),
                            depth: 1,
                        },
                        data,
                        ImageState {
                            queue,
                            stage: hal::pso::PipelineStage::FRAGMENT_SHADER,
                            access: hal::image::Access::SHADER_READ,
                            layout: hal::image::Layout::General,
                        },
                        ImageState {
                            queue,
                            stage: hal::pso::PipelineStage::FRAGMENT_SHADER,
                            access: hal::image::Access::SHADER_READ,
                            layout: hal::image::Layout::General,
                        },
                    )
                    .unwrap();
            }},
            |glyph| {
                // TODO: dont display glyph if out of screen bounds

                let uvs = glyph.tex_coords;
                //let pos = glyph.pixel_coords;
                let pos = glyph.pixel_coords;
                let color: [f32;4] = glyph.color;

                println!("drawing to {:?}", pos);
                
                (
                    glyph.z.to_bits(),
                    vec![
                        TextVertex {
                            position: [pos.min.x as f32, pos.min.y as f32].into(),
                            uv: [uvs.min.x, uvs.min.y].into(),
                            color: color.into(),
                        },
                        TextVertex {
                            position: [pos.max.x as f32, pos.min.y as f32].into(),
                            uv: [uvs.max.x, uvs.min.y].into(),
                            color: color.into(),
                        },
                        TextVertex {
                            position: [pos.max.x as f32, pos.max.y as f32].into(),
                            uv: [uvs.max.x, uvs.max.y].into(),
                            color: color.into(),
                        },
                        TextVertex {
                            position: [pos.min.x as f32, pos.min.y as f32].into(),
                            uv: [uvs.min.x, uvs.min.y].into(),
                            color: color.into(),
                        },
                        TextVertex {
                            position: [pos.min.x as f32, pos.max.y as f32].into(),
                            uv: [uvs.min.x, uvs.max.y].into(),
                            color: color.into(),
                        },
                        TextVertex {
                            position: [pos.max.x as f32, pos.max.y as f32].into(),
                            uv: [uvs.max.x, uvs.max.y].into(),
                            color: color.into(),
                        },
                    ],
                )
            },
        );
        match action {
            Ok(BrushAction::Draw(vertices)) => {
                text_vertex_container.0 = vertices
                    .into_iter()
                    .flat_map(|(_id, verts)| verts.into_iter())
                    .collect();
            },
            Ok(BrushAction::ReDraw) => { println!("redraw"); },
            Err(BrushError::TextureTooSmall { suggested }) => { println!("brusherror. Suggest {:?}", suggested); },
        }
    }
}

fn create_glyph_texture<B: Backend>(
    factory: &mut Factory<B>,
    queue: QueueId,
    width: u32,
    height: u32,
) -> Texture {
    use hal::format::{Component as C, Swizzle};
    TextureBuilder::new()
        .with_kind(hal::image::Kind::D2(width, height, 1, 1))
        .with_view_kind(hal::image::ViewKind::D2)
        .with_data_width(width)
        .with_data_height(height)
        .with_data(vec![R8Unorm { repr: [0] }; (width * height) as _])
        // This swizzle is required when working with `R8Unorm` on metal.
        // Glyph texture is biased towards 1.0 using "color_bias" attribute instead.
        .with_swizzle(Swizzle(C::Zero, C::Zero, C::Zero, C::R))
        .build(
            ImageState {
                queue,
                stage: hal::pso::PipelineStage::FRAGMENT_SHADER,
                access: hal::image::Access::SHADER_READ,
                layout: hal::image::Layout::General,
            },
            factory,
        )
        .map(B::wrap_texture)
        .expect("Failed to create glyph texture")
}
