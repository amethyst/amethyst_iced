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
    Align, Button, ButtonState, Column, Container, Element, IcedBundle, IcedUI, Length, Sandbox, SandboxContainer, Text, pane_grid, PaneGrid, HorizontalAlignment, VerticalAlignment,
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
                    .with_clear([0.1,0.1,0.1,1.0])
                )
                .with_plugin(IcedUI::default()),
        )?
        .with_bundle(IcedBundle::<PaneGridUIState>::default())?;

    let mut game = Application::new(assets, PaneGridState::default(), game_data)?;
    game.run();

    Ok(())
}

#[derive(Default)]
struct PaneGridState;

impl SimpleState for PaneGridState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        world.insert(SandboxContainer::new(PaneGridUIState::default()));
    }
}

#[derive(Debug)]
struct PaneGridUIState {
    panes: pane_grid::State<Content>,
    panes_created: usize,
}

impl Default for PaneGridUIState {
    fn default() -> Self {
        let (panes, _) = pane_grid::State::new(Content::new(0));

        PaneGridUIState {
            panes,
            panes_created: 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum PaneGridUIMessage {
    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    FocusAdjacent(pane_grid::Direction),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    Close(pane_grid::Pane),
    CloseFocused,
}

impl Sandbox for PaneGridUIState {
    type UIMessage = PaneGridUIMessage;
    type GameMessage = ();

    fn view(&mut self) -> Element<Self::UIMessage> {
        let total_panes = self.panes.len();

        let pane_grid =
            PaneGrid::new(&mut self.panes, |pane, content, focus| {
                content.view(pane, focus, total_panes)
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(10)
            .on_drag(PaneGridUIMessage::Dragged)
            .on_resize(PaneGridUIMessage::Resized);

        Container::new(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }

    fn update(&mut self, message: &Self::UIMessage) -> Vec<Self::GameMessage> {
         match message {
            PaneGridUIMessage::Split(axis, pane) => {
                let _ = self.panes.split(
                    *axis,
                    &pane,
                    Content::new(self.panes_created),
                );

                self.panes_created += 1;
            }
            PaneGridUIMessage::SplitFocused(axis) => {
                if let Some(pane) = self.panes.active() {
                    let _ = self.panes.split(
                        *axis,
                        &pane,
                        Content::new(self.panes_created),
                    );

                    self.panes_created += 1;
                }
            }
            PaneGridUIMessage::FocusAdjacent(direction) => {
                if let Some(pane) = self.panes.active() {
                    if let Some(adjacent) =
                        self.panes.adjacent(&pane, *direction)
                    {
                        self.panes.focus(&adjacent);
                    }
                }
            }
            PaneGridUIMessage::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(&split, *ratio);
            }
            PaneGridUIMessage::Dragged(pane_grid::DragEvent::Dropped {
                pane,
                target,
            }) => {
                self.panes.swap(&pane, &target);
            }
            PaneGridUIMessage::Dragged(_) => {}
            PaneGridUIMessage::Close(pane) => {
                let _ = self.panes.close(&pane);
            }
            PaneGridUIMessage::CloseFocused => {
                if let Some(pane) = self.panes.active() {
                    let _ = self.panes.close(&pane);
                }
            }
        }
        Vec::new() 
    }
}


#[derive(Debug)]
struct Content {
    id: usize,
    split_horizontally: ButtonState,
    split_vertically: ButtonState,
    close: ButtonState,
}

impl Content {
    fn new(id: usize) -> Self {
        Content {
            id,
            split_horizontally: ButtonState::new(),
            split_vertically: ButtonState::new(),
            close: ButtonState::new(),
        }
    }
    fn view(
        &mut self,
        pane: pane_grid::Pane,
        focus: Option<pane_grid::Focus>,
        total_panes: usize,
    ) -> Element<PaneGridUIMessage> {
        let Content {
            id,
            split_horizontally,
            split_vertically,
            close,
        } = self;

        let mut controls = Column::new()
            .spacing(5)
            .max_width(150)
            .push(
                Button::new(
                    split_horizontally,
                    Text::new("Split horizontally"),
                )
                .width(Length::Fill)
                .padding(8)
                .on_press(PaneGridUIMessage::Split(pane_grid::Axis::Horizontal, pane))
            )
            .push(
                Button::new(
                    split_vertically,
                    Text::new("Split vertically"),
                )
                .width(Length::Fill)
                .padding(8)
                .on_press(PaneGridUIMessage::Split(pane_grid::Axis::Vertical, pane))
            );
            
            if total_panes > 1 {
                controls = controls.push(
                    Button::new(
                        close, 
                        Text::new("Close"),
                    )
                    .width(Length::Fill)
                    .padding(8)
                    .on_press(PaneGridUIMessage::Close(pane))
            );
        }

        let content = Column::new()
            .width(Length::Fill)
            .spacing(10)
            .align_items(Align::Center)
            .push(Text::new(format!("Pane {}", id)).size(30))
            .push(controls);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_y()
            .into()
    }
}