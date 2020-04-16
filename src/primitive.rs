use crate::custom_widget::ImageHandle;
use crate::pass::IcedPass;
use crate::vertex::{ImageVertex, TriangleVertex};
use amethyst::ecs::{SystemData, World, WriteExpect};
use amethyst::renderer::{rendy::factory::Factory, rendy::hal, types::Backend};
use glsl_layout::vec4;
use glyph_brush::{rusttype::Scale, HorizontalAlign, Layout, Section, VerticalAlign};
use iced_native::{Color, HorizontalAlignment, Rectangle};

use crate::IcedGlyphBrush;

#[allow(dead_code)]
pub enum AmethystIcedPrimitive {
    Quad(Rectangle, Option<Color>),
    Image(Rectangle, ImageHandle),
    Text {
        bounds: Rectangle,
        content: String,
        size: u16,
        color: [f32; 4],
        horizontal_alignment: HorizontalAlignment,
    },
    Group(Vec<AmethystIcedPrimitive>),
    None,
}

/// Wrapper struct meant to avoid an user from interfering (accidentally or not)
/// into amethyst_iced's primitives
pub(crate) struct IcedPrimitives(pub(crate) Option<AmethystIcedPrimitive>, pub u64);

impl Default for IcedPrimitives {
    fn default() -> Self {
        IcedPrimitives(None, 0)
    }
}

impl AmethystIcedPrimitive {
    /// Consumes the Primitive, rendering it on the pass
    pub fn render<B: Backend>(
        self,
        pass: &mut IcedPass<B>,
        factory: &Factory<B>,
        index: usize,
        world: &World,
    ) {
        match self {
            AmethystIcedPrimitive::Group(primitives) => primitives.into_iter().for_each(|p| {
                p.render(pass, factory, index, world);
            }),
            AmethystIcedPrimitive::Quad(bounds, color) => {
                let iced_color = color.unwrap_or(Color::WHITE);
                let color: vec4 = [iced_color.r, iced_color.g, iced_color.b, iced_color.a].into();

                pass.triangle_pipeline.vertices.extend_from_slice(&[
                    TriangleVertex {
                        position: [bounds.x, bounds.y].into(),
                        color,
                    },
                    TriangleVertex {
                        position: [bounds.x + bounds.width, bounds.y].into(),
                        color,
                    },
                    TriangleVertex {
                        position: [bounds.x + bounds.width, bounds.y + bounds.height].into(),
                        color,
                    },
                    TriangleVertex {
                        position: [bounds.x, bounds.y].into(),
                        color,
                    },
                    TriangleVertex {
                        position: [bounds.x, bounds.y + bounds.height].into(),
                        color,
                    },
                    TriangleVertex {
                        position: [bounds.x + bounds.width, bounds.y + bounds.height].into(),
                        color,
                    },
                ]);
            }
            AmethystIcedPrimitive::Image(bounds, handle) => match handle {
                ImageHandle::Texture { handle, .. } => {
                    let info = pass.image_pipeline.textures.insert(
                        factory,
                        world,
                        &handle,
                        hal::image::Layout::ShaderReadOnlyOptimal,
                    );
                    if let Some((id, _changed)) = info {
                        let verts = vec![
                            ImageVertex {
                                position: [bounds.x, bounds.y].into(),
                                uv: [0., 0.].into(),
                            },
                            ImageVertex {
                                position: [bounds.x + bounds.width, bounds.y].into(),
                                uv: [1., 0.].into(),
                            },
                            ImageVertex {
                                position: [bounds.x + bounds.width, bounds.y + bounds.height]
                                    .into(),
                                uv: [1., 1.].into(),
                            },
                            ImageVertex {
                                position: [bounds.x, bounds.y].into(),
                                uv: [0., 0.].into(),
                            },
                            ImageVertex {
                                position: [bounds.x, bounds.y + bounds.height].into(),
                                uv: [0., 1.].into(),
                            },
                            ImageVertex {
                                position: [bounds.x + bounds.width, bounds.y + bounds.height]
                                    .into(),
                                uv: [1., 1.].into(),
                            },
                        ];
                        pass.image_pipeline.batches.insert(id, verts);
                    }
                }
            },
            AmethystIcedPrimitive::Text {
                content,
                color,
                size,
                bounds,
                horizontal_alignment,
            } => {
                let mut iced_glyph_brush = WriteExpect::<'_, IcedGlyphBrush>::fetch(world);
                iced_glyph_brush.queue(Section {
                    text: &content,
                    color,
                    scale: Scale::uniform(size as f32),
                    bounds: (bounds.width, bounds.height),
                    screen_position: (bounds.x, bounds.y),
                    layout: Layout::default()
                        .h_align(into_h_align(horizontal_alignment))
                        // Todo: support proper Vertical alignment
                        .v_align(VerticalAlign::Top),
                    //.v_align(into_v_align(vertical_alignment)),
                    ..Default::default()
                });
            }
            AmethystIcedPrimitive::None => {}
        }
    }
}

pub fn into_h_align(align: HorizontalAlignment) -> HorizontalAlign {
    match align {
        HorizontalAlignment::Left => HorizontalAlign::Left,
        HorizontalAlignment::Center => HorizontalAlign::Center,
        HorizontalAlignment::Right => HorizontalAlign::Right,
    }
}

/*
pub fn into_v_align(align: VerticalAlignment) -> VerticalAlign {
    match align {
        VerticalAlignment::Top => VerticalAlign::Top,
        VerticalAlignment::Center => VerticalAlign::Center,
        VerticalAlignment::Bottom => VerticalAlign::Bottom,
    }
}
*/
