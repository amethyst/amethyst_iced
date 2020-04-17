use iced_native::button::Renderer;
use iced_native::{Element, Layout, Point, Rectangle};

use crate::backend::IcedRenderer;
use crate::primitive::AmethystIcedPrimitive;
use crate::{BorderStyle, style::colors};

impl<'a> Renderer for IcedRenderer<'a> {
    const DEFAULT_PADDING: u16 = 5;
    type Style = ButtonStyle;

    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        bounds: Rectangle,
        cursor_position: Point,
        is_disabled: bool,
        is_pressed: bool,
        style: &Self::Style,
        content: &Element<'_, Message, Self>,
        content_layout: Layout<'_>,
    ) -> Self::Output {
        // TODO: Make these default colors customizable
        // + add color to style
        // + handle hovering
        let background = match style {
            ButtonStyle::Builtin {
                background_color,
                hovered_color, 
                pressed_color,
                disabled_color,
                ..
            } => {
                let color = if is_disabled {
                    *disabled_color
                } else if is_pressed {
                    *pressed_color 
                } else if bounds.contains(cursor_position) {
                    *hovered_color
                } else {
                    *background_color
                };
                AmethystIcedPrimitive::Quad(bounds, Some(color.into()))
            }
        };
        let children = content.draw(self, defaults, content_layout, cursor_position);

        AmethystIcedPrimitive::Group(vec![background, children])
    }
}

#[derive(Clone)]
pub enum ButtonStyle {
    Builtin {
        background_color: [f32;4],
        hovered_color: [f32;4],
        pressed_color: [f32;4],
        disabled_color: [f32;4],
        border_radius: u32,
        border: BorderStyle,
    },
} 

impl ButtonStyle {
    pub fn danger() -> Self {
        ButtonStyle::Builtin {
            background_color: colors::DANGER, 
            hovered_color: colors::DANGER_SHADED, 
            disabled_color: colors::DANGER_SHADED, 
            pressed_color: colors::DANGER_DARKER, 
            border_radius: 0,
            border: BorderStyle {
                width: 1,
                color: [0.,0.,0.,1.],
            },
        }
    }
    
    pub fn primary() -> Self {
        ButtonStyle::Builtin {
            background_color: colors::PRIMARY, 
            hovered_color: colors::PRIMARY_SHADED, 
            disabled_color: colors::PRIMARY_SHADED, 
            pressed_color: colors::PRIMARY_DARKER, 
            border_radius: 0,
            border: BorderStyle {
                width: 1,
                color: [0.,0.,0.,1.],
            },
        }
    }
}

impl Default for ButtonStyle {
    fn default() -> Self {
        ButtonStyle::Builtin {
            background_color: colors::VERY_LIGHT_GRAY, 
            hovered_color: colors::LIGHT_GRAY, 
            disabled_color: colors::LIGHT_GRAY, 
            pressed_color: colors::GRAY, 
            border_radius: 0,
            border: BorderStyle {
                width: 1,
                color: [0.,0.,0.,1.],
            },
        }
    }
}