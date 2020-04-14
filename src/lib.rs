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
pub mod widget;

pub use bundle::IcedBundle;
pub use custom_widget::*;
pub use plugin::IcedUI;
pub use sandbox::{Element, Sandbox, SandboxContainer};

// Conveniently re-exports iced's Widget types
pub use iced_native::{Align, button::State as ButtonState, Color, Length, Text};

pub use widget::*;



pub type IcedGlyphBrush = glyph_brush::GlyphBrush<'static, (u32, Vec<crate::vertex::TextVertex>)>;
