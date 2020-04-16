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
    Align, Column, Container, Element, IcedBundle, IcedUI, Length, Checkbox, Sandbox,
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
        .with_bundle(IcedBundle::<CheckboxUIState>::default())?;

    let mut game = Application::new(assets, CheckboxState::default(), game_data)?;
    game.run();

    Ok(())
}

#[derive(Default)]
struct CheckboxState;

impl SimpleState for CheckboxState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        world.insert(SandboxContainer::new(CheckboxUIState::default()));
    }
}

#[derive(Default, Debug)]
struct CheckboxUIState {
    likes_chocolate: bool,
    likes_pasta: bool,
    likes_pizza: bool,
    likes_tartiflette: bool,
    likes_sausage: bool,
}

#[derive(Clone)]
enum CheckboxUIMessage {
    ToggleChocolate(bool),
    TogglePasta(bool),
    TogglePizza(bool),
    ToggleTartiflette(bool),
    ToggleSausage(bool),
}

impl Sandbox for CheckboxUIState {
    type UIMessage = CheckboxUIMessage;
    type GameMessage = ();

    fn view(&mut self) -> Element<Self::UIMessage> {
        let col = Column::new()
            .align_items(Align::Center)
            .push(Text::new("I love ..."))
            .push(
                Column::new()
                    .spacing(5)
                    .width(Length::Units(400))
                    .push(Checkbox::new(
                        self.likes_chocolate,
                        "Chocolate",
                        CheckboxUIMessage::ToggleChocolate,
                    ))
                    .push(Checkbox::new(
                        self.likes_pasta,
                        "Pasta",
                        CheckboxUIMessage::TogglePasta,
                    ))
                    .push(Checkbox::new(
                        self.likes_pizza,
                        "Pizza",
                        CheckboxUIMessage::TogglePizza,
                    ))
                    .push(Checkbox::new(
                        self.likes_tartiflette,
                        "Tartiflette",
                        CheckboxUIMessage::ToggleTartiflette,
                    ))
                    .push(Checkbox::new(
                        self.likes_sausage,
                        "Sausage",
                        CheckboxUIMessage::ToggleSausage,
                    ))
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
            CheckboxUIMessage::ToggleChocolate(v) => self.likes_chocolate = *v, 
            CheckboxUIMessage::TogglePasta(v) => self.likes_pasta = *v, 
            CheckboxUIMessage::TogglePizza(v) => self.likes_pizza = *v, 
            CheckboxUIMessage::ToggleTartiflette(v) => self.likes_tartiflette = *v, 
            CheckboxUIMessage::ToggleSausage(v) => self.likes_sausage = *v, 
        }
        vec![]
    }
}
