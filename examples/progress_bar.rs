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
    Align, Button, ButtonState, Column, Container, Element, IcedBundle, IcedUI, Length, Sandbox,
    SandboxContainer, Text, ProgressBar,
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
        .with_bundle(IcedBundle::<ProgressBarUIState>::default())?;

    let mut game = Application::new(assets, ProgressBarState::default(), game_data)?;
    game.run();

    Ok(())
}

#[derive(Default)]
struct ProgressBarState;

impl SimpleState for ProgressBarState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        world.insert(SandboxContainer::new(ProgressBarUIState::default()));
    }
}

#[derive(Default, Debug)]
struct ProgressBarUIState {
    clicks: u32,
    button_state: ButtonState,
}

#[derive(Clone)]
enum ProgressBarUIMessage {
    Clicked,
}

impl Sandbox for ProgressBarUIState {
    type UIMessage = ProgressBarUIMessage;
    type GameMessage = ();

    fn view(&mut self) -> Element<Self::UIMessage> {
        let col = Column::new()
            .align_items(Align::Center)
            .spacing(5)
            .push(Text::new(format!("Pressed {}/10 times", self.clicks)))
            .push(ProgressBar::new(0.0..=10., self.clicks as f32).width(Length::Units(400)))
            .push(
                Button::new(
                    &mut self.button_state,
                    Text::new("Click me !"),
                )
                .on_press(ProgressBarUIMessage::Clicked),
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
            ProgressBarUIMessage::Clicked => self.clicks = (self.clicks+1).min(10),
        }
        vec![]
    }
}
