use amethyst::{
    prelude::*,
    renderer::{
        plugins::RenderToWindow,
        //types::DefaultBackend,
        rendy::util::vulkan::Backend,
        RenderingBundle,
    },
    utils::application_root_dir,
    Error,
};
use amethyst_iced::{
    Align, Column, Container, Element, IcedBundle, IcedUI, Length, Radio, Sandbox,
    SandboxContainer, Text,
};

fn main() -> Result<(), Error> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let assets = app_root.join("assets");
    let display_config = assets.join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<Backend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.1, 0.1, 0.1, 1.0]),
                )
                .with_plugin(IcedUI::default()),
        )?
        .with_bundle(IcedBundle::<RadioUIState>::default())?;

    let mut game = Application::new(assets, RadioState::default(), game_data)?;
    game.run();

    Ok(())
}

#[derive(Default)]
struct RadioState;

impl SimpleState for RadioState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        world.insert(SandboxContainer::new(RadioUIState::default()));
    }
}

#[derive(Default, Debug)]
struct RadioUIState {
    choice: Option<Choice>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice {
    C,
    Python,
    CSharp,
    Rust,
    Java,
}

#[derive(Clone)]
enum RadioUIMessage {
    Selected(Choice),
}

impl Sandbox for RadioUIState {
    type UIMessage = RadioUIMessage;
    type GameMessage = ();

    fn view(&mut self) -> Element<Self::UIMessage> {
        let col = Column::new()
            .align_items(Align::Center)
            .push(Text::new("Amethyst_Iced is written in ..."))
            .push(
                Column::new()
                    .spacing(5)
                    .width(Length::Units(400))
                    .push(Radio::new(
                        Choice::C,
                        "C",
                        self.choice,
                        RadioUIMessage::Selected,
                    ))
                    .push(Radio::new(
                        Choice::Python,
                        "Python",
                        self.choice,
                        RadioUIMessage::Selected,
                    ))
                    .push(Radio::new(
                        Choice::CSharp,
                        "C#",
                        self.choice,
                        RadioUIMessage::Selected,
                    ))
                    .push(Radio::new(
                        Choice::Rust,
                        "Rust",
                        self.choice,
                        RadioUIMessage::Selected,
                    ))
                    .push(Radio::new(
                        Choice::Java,
                        "Java",
                        self.choice,
                        RadioUIMessage::Selected,
                    )))
                    .push(Text::new(if self.choice == Some(Choice::Rust) {
                        "right!"
                    } else {
                        "wrong."
                    }),
            );

        Container::new(col)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn update(&mut self, message: &Self::UIMessage) -> Vec<Self::GameMessage> {
        match message {
            RadioUIMessage::Selected(choice) => {
                self.choice = Some(*choice);
            }
        }
        vec![]
    }
}
