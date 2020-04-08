use glsl_layout::{vec2, AsStd140};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, AsStd140)]
#[repr(C, align(4))]
pub struct IcedUniform {
    /// Size of the window available to the Iced Backend
    inverse_window_size: vec2,
}
