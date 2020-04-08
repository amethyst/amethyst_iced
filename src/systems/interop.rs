use crate::sandbox::{Sandbox, SandboxContainer};
use amethyst::ecs::{Read, System, SystemData, World, Write};
use amethyst::shrev::{EventChannel, ReaderId};

/// The system in charge of interop between the Iced Sandbox, and the World.
/// Reads UIMessages and sends back GameMessages to other systems.
pub(crate) struct IcedInteropSystem<S: Sandbox> {
    sandbox: std::marker::PhantomData<S>,
    ui_event_reader: Option<ReaderId<S::UIMessage>>,
}

impl<S: Sandbox> Default for IcedInteropSystem<S> {
    fn default() -> Self {
        IcedInteropSystem {
            sandbox: std::marker::PhantomData,
            ui_event_reader: None,
        }
    }
}

impl<'a, S: Sandbox> System<'a> for IcedInteropSystem<S> {
    type SystemData = (
        Read<'a, EventChannel<S::UIMessage>>,
        Write<'a, EventChannel<S::GameMessage>>,
        Write<'a, SandboxContainer<S>>,
    );

    fn run(&mut self, (ui_messages, mut game_messages, mut sandbox): Self::SystemData) {
        let reader = self
            .ui_event_reader
            .as_mut()
            .expect("Failed to get ReaderID: IcedInteropSystem has not been setup.");
        ui_messages
            .read(reader)
            .flat_map(|ui_msg| sandbox.update(ui_msg))
            .for_each(|game_msg| game_messages.single_write(game_msg));
    }

    fn setup(&mut self, res: &mut World) {
        Self::SystemData::setup(res);
        let mut ui_event_channel = Write::<'_, EventChannel<S::UIMessage>>::fetch(res);
        self.ui_event_reader = Some(ui_event_channel.register_reader());
    }
}
