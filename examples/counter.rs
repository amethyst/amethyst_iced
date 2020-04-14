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
    Button, ButtonState, Column, Container, Element, IcedBundle, IcedUI, Length, Sandbox, SandboxContainer, Text
}; 

fn main() -> Result<(), Error> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let assets = app_root.join("assets");
    let display_config = assets.join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<Backend>::new()
                .with_plugin(RenderToWindow::from_config_path(display_config)?)
                .with_plugin(IcedUI::default()),
        )?
        .with_bundle(IcedBundle::<CounterUIState>::default())?;

    let mut game = Application::new(assets, CounterState::default(), game_data)?;
    game.run();

    Ok(())
}

#[derive(Default)]
struct CounterState;

impl SimpleState for CounterState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        world.insert(SandboxContainer::new(CounterUIState::default()));
    }
}

#[derive(Default, Debug)]
struct CounterUIState {
    pressed: u32,
    button_state: ButtonState, 
}

#[derive(Clone)]
enum CounterUIMessage {
    Clicked,
}

impl Sandbox for CounterUIState {
    type UIMessage = CounterUIMessage;
    type GameMessage = ();

    fn view(&mut self) -> Element<Self::UIMessage> {
        let col = Column::new()
            .push(
                Text::new(format!("Pressed {} times", self.pressed))
            )
            .push(
                Button::<'_, '_, Self::UIMessage>::new(&mut self.button_state, Text::new("Click me !"))
                .on_press(CounterUIMessage::Clicked)
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
            CounterUIMessage::Clicked => {
                println!("clicked");
                self.pressed += 1;
            } ,
        }
        vec![]
    }
}