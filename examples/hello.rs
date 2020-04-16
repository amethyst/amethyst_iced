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
    Color, Column, Container, Element, IcedBundle, IcedUI, Length, Sandbox, SandboxContainer, Text,
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
        .with_bundle(IcedBundle::<HelloUIState>::default())?;

    let mut game = Application::new(assets, HelloState::default(), game_data)?;
    game.run();

    Ok(())
}

#[derive(Default)]
struct HelloState;

impl SimpleState for HelloState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        world.insert(SandboxContainer::new(HelloUIState::default()));
    }
}

#[derive(Default, Debug)]
struct HelloUIState;

impl Sandbox for HelloUIState {
    type UIMessage = u32;
    type GameMessage = ();

    fn view(&mut self) -> Element<Self::UIMessage> {
        let col = Column::new()
            .spacing(5)
            .push(
                Text::new("Test red")
                    .size(40)
                    .color(Color::from_rgb(1., 0., 0.)),
            )
            .push(Text::new("Test white").color(Color::from_rgb(1., 1., 1.)))
            .push(Text::new("Test green").color(Color::from_rgb(0., 1., 0.)))
            .push(Text::new("Test blue").color(Color::from_rgb(0., 0., 1.)))
            .push(Text::new("Test yellow").color(Color::from_rgb(1., 1., 0.)));

        Container::new(col)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
