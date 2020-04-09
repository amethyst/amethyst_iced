use amethyst::renderer::rendy::{
    hal::format::Format,
    mesh::{AsVertex, VertexFormat},
};
use glsl_layout::{vec2, vec4, AsStd140};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, AsStd140)]
#[repr(C, align(4))]
pub struct TriangleVertex {
    pub position: vec2,
    pub color: vec4,
}

impl AsVertex for TriangleVertex {
    fn vertex() -> VertexFormat {
        VertexFormat::new((
            (Format::Rg32Sfloat, "in_pos"),
            (Format::Rgba32Sfloat, "in_color"),
        ))
    }
}
