use crate::pass::IcedPass;
use crate::vertex::IcedVertex;
use amethyst::renderer::{rendy::factory::Factory, types::Backend};
use iced_native::{Color, Rectangle};

#[allow(dead_code)]
pub enum AmethystIcedPrimitive {
    Quad(Rectangle, Option<Color>),
    Image,
    Text,
    Group(Vec<AmethystIcedPrimitive>),
}

/// Wrapper struct meant to avoid an user from interfering (accidentally or not) 
/// into amethyst_iced's primitives 
pub(crate) struct IcedPrimitives(pub(crate) Option<AmethystIcedPrimitive>);

impl Default for IcedPrimitives {
    fn default() -> Self {
        IcedPrimitives(None)
    }
}

impl AmethystIcedPrimitive {
    /// Consumes the Primitive, rendering it on the pass
    pub fn render<B: Backend>(
        self,
        pass: &mut IcedPass<B>,
        factory: &Factory<B>,
        index: usize,
    ) {
        let map_x = |value: f32| value * 2. / 800. - 1.;
        let map_y = |value: f32| value * 2. / 600. - 1.;
        match self {
            AmethystIcedPrimitive::Group(primitives) => primitives
                .into_iter()
                .for_each(|p| {
                    p.render(pass, factory, index);
                }),
            AmethystIcedPrimitive::Quad(bounds, color) => {
                let iced_color = color.unwrap_or(Color::WHITE);
                let color = [iced_color.r, iced_color.g, iced_color.b, iced_color.a].into();
                let vertex_buffer = pass.get_vertex_buffer_mut();
                vertex_buffer.write(
                    factory,
                    index,
                    6,
                    Some(&[
                        IcedVertex {
                            position: [map_x(bounds.x), map_y(bounds.y)].into(),
                            color,
                        },
                        IcedVertex {
                            position: [map_x(bounds.x + bounds.width), map_y(bounds.y)].into(),
                            color,
                        },
                        IcedVertex {
                            position: [
                                map_x(bounds.x + bounds.width),
                                map_y(bounds.y + bounds.height),
                            ]
                            .into(),
                            color,
                        },
                        IcedVertex {
                            position: [map_x(bounds.x), map_y(bounds.y)].into(),
                            color,
                        },
                        IcedVertex {
                            position: [map_x(bounds.x), map_y(bounds.y + bounds.height)].into(),
                            color,
                        },
                        IcedVertex {
                            position: [
                                map_x(bounds.x + bounds.width),
                                map_y(bounds.y + bounds.height),
                            ]
                            .into(),
                            color,
                        },
                    ]),
                );
            }
            _ => unimplemented!(),
        }
    }
}
