use amethyst::renderer::rendy::{
    hal::format::Format,
    mesh::{AsVertex, VertexFormat},
};
use glsl_layout::{vec2, AsStd140};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, AsStd140)]
#[repr(C, align(4))]
pub struct TextVertex {
    pub position: vec2,
    pub uv: vec2,
}

impl AsVertex for TextVertex {
    fn vertex() -> VertexFormat {
        VertexFormat::new((
            (Format::Rg32Sfloat, "in_pos"),
            (Format::Rg32Sfloat, "in_uv"),
        ))
    }
}
