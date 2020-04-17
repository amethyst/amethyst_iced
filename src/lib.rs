mod backend;
mod bundle;
mod custom_widget;
mod pass;
mod pipelines;
mod plugin;
mod primitive;
mod resources;
pub mod sandbox;
mod systems;
mod uniform;
mod vertex;
pub mod widget;
pub mod style;

pub use bundle::IcedBundle;
pub use custom_widget::*;
pub use plugin::IcedUI;
pub use sandbox::{Element, Sandbox, SandboxContainer};

// Conveniently re-exports iced's Widget types
pub use iced_native::{
    button::State as ButtonState, slider::State as SliderState, pane_grid::self, Align, Color, Length, Text, HorizontalAlignment, VerticalAlignment, Font
};

pub use widget::*;
pub use style::*;
pub use resources::*;

pub type IcedGlyphBrush = glyph_brush::GlyphBrush<'static, (u32, Vec<crate::vertex::TextVertex>)>;
