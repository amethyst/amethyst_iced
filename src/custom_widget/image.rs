use iced_native::Hasher;
use std::hash::Hash;

use amethyst::assets::Handle;
use amethyst::renderer::{Texture};
use iced_native::{Element, Length, Widget, Layout, layout, Renderer, Point, Size};

use crate::{primitive::AmethystIcedPrimitive, backend::IcedRenderer};

pub struct Image {
    handle: ImageHandle,
    width: Length,
    height: Length,
}

#[derive(Hash, Clone)]
pub enum ImageHandle {
    Texture { handle: Handle<Texture>, width: u32, height: u32 },
}

impl From<(Handle<Texture>, u32, u32)> for ImageHandle {
    fn from((handle, width, height): (Handle<Texture>, u32, u32)) -> Self {
        ImageHandle::Texture{ handle, width, height }
    }
}

impl ImageHandle {
    pub fn dimensions(&self) -> (u32,u32) {
       match self {
            ImageHandle::Texture { width, height, .. } => (*width, *height), 
       } 
    }
}

impl Image {
    pub fn new<T: Into<ImageHandle>>(handle: T) -> Self {
        Image {
            handle: handle.into(),
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }
}

impl<Message> Widget<Message, IcedRenderer> for Image {
    fn width(&self) -> Length {
        self.width.clone()
    }

    fn height(&self) -> Length {
        self.height.clone()
    }

    fn layout(
        &self,
        _renderer: &IcedRenderer, 
        limits: &layout::Limits,
    ) -> layout::Node {
        let (width, height) = self.handle.dimensions();
        let aspect_ratio = width as f32 / height as f32;
        let mut size = limits
            .width(self.width)
            .height(self.height)
            .resolve(Size::new(width as f32, height as f32));
        let viewport_aspect_ratio = size.width / size.height;
        if viewport_aspect_ratio > aspect_ratio {
            size.width = width as f32 * size.height / height as f32;
        } else {
            size.height = height as f32 * size.width / width as f32;
        }

        layout::Node::new(size)
    }

    fn draw(&self, _renderer: &mut IcedRenderer, _defaults: &<IcedRenderer as Renderer>::Defaults, layout: Layout<'_>, _cursor_position: Point) -> <IcedRenderer as Renderer>::Output {
        let bounds = layout.bounds();
        AmethystIcedPrimitive::Image(
            bounds, 
            self.handle.clone(),
        )
    }
    
    fn hash_layout(&self, state: &mut Hasher) {
        std::any::TypeId::of::<Image>().hash(state);

        self.handle.hash(state);
        self.width.hash(state);
        self.height.hash(state);
    }
}

impl<'a, Message> From<Image> for Element<'a, Message, IcedRenderer> {
    fn from(image: Image) -> Element<'a, Message, IcedRenderer> {
        Element::new(image)
    }
}