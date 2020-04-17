use amethyst::assets::AssetStorage;
use amethyst::ecs::{Read, ReadExpect, System, SystemData, World, Write, WriteExpect};
use amethyst::renderer::SpriteSheet;
use amethyst::shrev::{EventChannel, ReaderId};
use amethyst::window::ScreenDimensions;
use amethyst::winit::{
    ElementState, Event as WinitEvent, MouseButton, WindowEvent as WinitWindowEvent,
};
use iced_native::{Cache, Size, UserInterface};

use crate::backend::IcedRenderer;
use crate::primitive::IcedPrimitives;
use crate::sandbox::{Sandbox, SandboxContainer};
use crate::resources::FontCache;

use crate::IcedGlyphBrush;

use std::hash::Hasher;

pub(crate) struct IcedDrawSystem<S: Sandbox> {
    _sandbox: std::marker::PhantomData<S>,
    winit_reader_id: Option<ReaderId<WinitEvent>>,
    cache: Option<Cache>,
}

impl<S: Sandbox> Default for IcedDrawSystem<S> {
    fn default() -> Self {
        IcedDrawSystem {
            _sandbox: std::marker::PhantomData,
            winit_reader_id: None,
            cache: Some(Cache::default()),
        }
    }
}

impl<'a, S: Sandbox> System<'a> for IcedDrawSystem<S> {
    type SystemData = (
        Read<'a, EventChannel<WinitEvent>>,
        Write<'a, EventChannel<<S as Sandbox>::UIMessage>>,
        Option<Write<'a, SandboxContainer<S>>>,
        Read<'a, AssetStorage<SpriteSheet>>,
        WriteExpect<'a, IcedGlyphBrush>,
        Read<'a, FontCache>,
        ReadExpect<'a, ScreenDimensions>,
        Write<'a, IcedPrimitives>,
    );

    fn run(
        &mut self,
        (
            winit_events,
            mut ui_messages,
            sandbox,
            sprite_sheet,
            glyph_brush,
            font_cache,
            screen_dimensions,
            mut iced_primitives,
        ): Self::SystemData,
    ) {
        if sandbox.is_none() {
            log::warn!("No sandbox was found in resources, Iced UI will not be drawn.");
            return;
        }
        let mut sandbox = sandbox.unwrap();
        {
            let mut renderer = IcedRenderer::new(sprite_sheet, glyph_brush, font_cache);

            let reader = self
                .winit_reader_id
                .as_mut()
                .expect("Failed to get ReaderID: IcedUpdateSystem has not been setup.");
            let bounds: Size = [screen_dimensions.width(), screen_dimensions.height()].into();
            let cache = self.cache.take().unwrap();
            let mut user_interface =
                UserInterface::build(sandbox.view(), bounds, cache, &mut renderer);
            winit_events
                .read(reader)
                .filter_map(|winit_event| match winit_event {
                    // TODO: Propper handling of window events, using iced_winit::conversion
                    // Possible when Amethyst upgrades to winit 0.22
                    WinitEvent::WindowEvent {
                        event: WinitWindowEvent::Resized(size),
                        ..
                    } => Some(iced_native::Event::Window(
                        iced_native::window::Event::Resized {
                            width: size.width as u32,
                            height: size.height as u32,
                        },
                    )),
                    WinitEvent::WindowEvent {
                        event:
                            WinitWindowEvent::MouseInput {
                                button: MouseButton::Left,
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } => Some(iced_native::Event::Mouse(
                        iced_native::input::mouse::Event::Input {
                            state: iced_native::input::ButtonState::Pressed,
                            button: iced_native::input::mouse::Button::Left,
                        },
                    )),
                    WinitEvent::WindowEvent {
                        event:
                            WinitWindowEvent::MouseInput {
                                button: MouseButton::Left,
                                state: ElementState::Released,
                                ..
                            },
                        ..
                    } => Some(iced_native::Event::Mouse(
                        iced_native::input::mouse::Event::Input {
                            state: iced_native::input::ButtonState::Released,
                            button: iced_native::input::mouse::Button::Left,
                        },
                    )),
                    WinitEvent::WindowEvent {
                        event: WinitWindowEvent::CursorMoved { position, .. },
                        ..
                    } => Some(iced_native::Event::Mouse(
                        iced_native::input::mouse::Event::CursorMoved {
                            x: position.x as f32,
                            y: position.y as f32,
                        },
                    )),
                    _ => None,
                })
                .flat_map(|iced_event| user_interface.update(vec![iced_event], None, &renderer))
                .for_each(|ui_msg| ui_messages.single_write(ui_msg));

            iced_primitives.0 = Some(user_interface.draw(&mut renderer));
            self.cache = Some(user_interface.into_cache());
        }
        let mut hasher = iced_native::Hasher::default();
        sandbox.hash_layout(&mut hasher);
        iced_primitives.1 = hasher.finish();
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        let mut winit_event_channel = Write::<'_, EventChannel<WinitEvent>>::fetch(world);
        self.winit_reader_id = Some(winit_event_channel.register_reader());
    }
}
