use crate::backend::IcedRenderer;

pub type Element<'a, 'r, Message> = iced_native::Element<'a, Message, IcedRenderer<'r>>;

/// The Sandbox is a basic UI wrapper.
///
/// This trait should be implemented by your UI State and defines
/// how the UI reacts to messages.
///
/// When receiving an UI message, the user can optionnaly send back a GameMessage
/// that their systems will listen to and react accordingly, allowing ECS interaction with the UI.
//
// Note: UIMessage & GameMessage have to be different types, otherwise the application will crash.
pub trait Sandbox: Send + Sync + 'static {
    type UIMessage: Send + Sync + 'static;
    type GameMessage: Send + Sync + 'static;

    fn update(&mut self, _message: &Self::UIMessage) -> Vec<Self::GameMessage> {
        vec![]
    }

    fn view(&self) -> Element<Self::UIMessage>;
}

#[derive(Default)]
/// The SandboxContainer is the structure that will store the Sandbox in the
/// ECS environement
pub struct SandboxContainer<S: Sandbox>(S);

impl<S: Sandbox> SandboxContainer<S> {
    pub fn new(sandbox: S) -> Self {
        SandboxContainer(sandbox)
    }
}

impl<S: Sandbox> Sandbox for SandboxContainer<S> {
    type UIMessage = S::UIMessage;
    type GameMessage = S::GameMessage;

    fn update(&mut self, message: &S::UIMessage) -> Vec<S::GameMessage> {
        self.0.update(message)
    }

    fn view(&self) -> Element<S::UIMessage> {
        self.0.view()
    }
}
