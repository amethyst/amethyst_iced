use amethyst::{
    prelude::*,
    assets::{AssetStorage, Loader},
    renderer::{
        plugins::RenderToWindow,
        //types::DefaultBackend,
        rendy::util::vulkan::Backend,
        RenderingBundle,
        Texture,
        ImageFormat,
    },
    utils::application_root_dir,
    Error,
    assets::Handle,
};
use amethyst_iced::{Align, Color, Column, Container, Element, IcedBundle, IcedUI, Length, Sandbox, SandboxContainer, Text, Image};

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
        .with_bundle(IcedBundle::<ImageUIState>::default())?;

    let mut game = Application::new(assets, ImageState::default(), game_data)?;
    game.run();

    Ok(())
}

struct ImageUIState {
    image: Handle<Texture>,
}

#[derive(Default, Debug)]
struct ImageState;

impl SimpleState for ImageState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;

    let image = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/test.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

        world.insert(SandboxContainer::new(ImageUIState {
            image, 
        }))
    }
}


impl Sandbox for ImageUIState {
    type UIMessage = u32;
    type GameMessage = ();

    fn view(&self) -> Element<Self::UIMessage> {
        let col = Column::new()
            .spacing(5)
            .align_items(Align::Center)
            .push(Text::new("Test red").color(Color::from_rgb(1., 0., 0.)))
            .push(Image::new((self.image.clone(), 64, 64)));

        Container::new(col)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}