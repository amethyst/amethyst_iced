mod button;
mod column;
mod container;
mod text;
mod slider;

use crate::backend::IcedRenderer;

pub type Button<'a, 'r, Message> = iced_native::Button<'a, Message, IcedRenderer<'r>>;
pub type Container<'a, 'r, Message> = iced_native::Container<'a, Message, IcedRenderer<'r>>;
pub type Column<'a, 'r, Message> = iced_native::Column<'a, Message, IcedRenderer<'r>>;
pub type Slider<'a, 'r, Message> = iced_native::Slider<'a, Message, IcedRenderer<'r>>;