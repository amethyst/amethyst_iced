mod backend;
mod bundle;
mod custom_widget;
mod pass;
mod pipelines;
mod plugin;
mod primitive;
pub mod sandbox;
mod systems;
mod uniform;
mod vertex;
mod widget;

pub use bundle::IcedBundle;
pub use plugin::IcedUI;
pub use sandbox::{Element, Sandbox, SandboxContainer};
pub use custom_widget::*;

// Conveniently re-exports iced's Widget types
pub use iced_native::{Align, Color, Column, Container, Length, Text};

pub type IcedGlyphBrush = glyph_brush::GlyphBrush<'static, (u32, crate::vertex::TextVertex)>;