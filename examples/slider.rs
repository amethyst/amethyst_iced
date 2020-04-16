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
    Align, Column, Container, Element, IcedBundle, IcedUI, Length, Sandbox, SandboxContainer,
    Slider, SliderState, Text,
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
        .with_bundle(IcedBundle::<SliderUIState>::default())?;

    let mut game = Application::new(assets, SliderExampleState::default(), game_data)?;
    game.run();

    Ok(())
}

#[derive(Default)]
struct SliderExampleState;

impl SimpleState for SliderExampleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        world.insert(SandboxContainer::new(SliderUIState::default()));
    }
}

#[derive(Default, Debug)]
struct SliderUIState {
    value: f32,
    state: SliderState,
}

#[derive(Clone)]
enum SliderUIMessage {
    Change(f32),
}

impl Sandbox for SliderUIState {
    type UIMessage = SliderUIMessage;
    type GameMessage = ();

    fn view(&mut self) -> Element<Self::UIMessage> {
        let col = Column::new()
            .align_items(Align::Center)
            .push(Text::new(format!("Value is : {}", self.value)))
            .push(
                Slider::new(
                    &mut self.state,
                    0.0..=100.,
                    self.value,
                    SliderUIMessage::Change,
                )
                .width(Length::Units(400)),
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
            SliderUIMessage::Change(val) => {
                self.value = *val;
            }
        }
        vec![]
    }
}
