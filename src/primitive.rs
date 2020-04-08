use crate::pass::IcedPass;
use crate::vertex::IcedVertex;
use crate::Sandbox;
use amethyst::renderer::{rendy::factory::Factory, types::Backend};
use iced_native::{Color, Rectangle};

#[allow(dead_code)]
pub enum AmethystIcedPrimitive {
    Quad(Rectangle, Option<Color>),
    Image,
    Text,
    Group(Vec<AmethystIcedPrimitive>),
}

impl AmethystIcedPrimitive {
    /// Consumes the Primitive, rendering it on the pass
    pub fn render<B: Backend, S: Sandbox + std::fmt::Debug>(
        self,
        pass: &mut IcedPass<B, S>,
        factory: &Factory<B>,
        index: usize,
    ) {
        let (width, height) = pass.get_fb_size();
        let map_x = |value: f32| value * 2. / width - 1.;
        let map_y = |value: f32| value * 2. / height - 1.;
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
