use iced::{
    button,
    Button,
    Settings,
    Clipboard,
    Application,
    Container,
    Column,
    Row,
    Element,
    Text,
    Command,
    ProgressBar,
    Length,
    Align,
    HorizontalAlignment,
    VerticalAlignment,
    Image,
    Svg,
};

use log::{ info, error };

use crate::{
    err::ConmxErr,
    gui,
    gui::style,
    dmx,
    conmx_core,
};

use super::components::Grid;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewMain {
    Fixtures,
    Direct,
}

#[derive(Debug, Clone, Copy)]
pub enum ConMXMsg {
    Grid,
    SwitchTab(ViewMain),
    ButtonPressed,
    SliderChange(usize, usize, u32),
    RandomChange,
}


#[derive(Debug)]
pub struct ConMX {
    grid: Grid,
    bstate: button::State,
    imgbstate: button::State,

    title: String,
    dmx: dmx::DMX,

    view: ViewMain,
}

impl ConMX {
    pub fn set_view(&mut self, new: ViewMain) {
        self.view = new;
    }

    pub fn get_view(&mut self) -> ViewMain {
        self.view.clone()
    }
}

impl Application for ConMX {
    type Executor = iced::executor::Default;
    type Message = ConMXMsg;
    type Flags = conmx_core::Config;

    fn new(config: conmx_core::Config) -> (Self, Command<ConMXMsg>) {
        let mut univ = dmx::DMX::new();
        univ.add_universe(dmx::DMXUniverse::new(0))
            .add_universe(dmx::DMXUniverse::new(0));
        let conmx = ConMX {
            title: String::from("This is a test window"),
            dmx: univ,
            view: ViewMain::Fixtures,

            grid: Grid::new(),
            imgbstate: button::State::new(),
            bstate: button::State::new(),
        };
        (conmx, Command::none())
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn update(&mut self, msg: ConMXMsg, _cb: &mut Clipboard) -> Command<Self::Message> {
        match msg {
            ConMXMsg::ButtonPressed => {
                info!("Button pressed");
            }
            ConMXMsg::SwitchTab(s) => {
                error!("switched tab to {:?}", s);
            }
            ConMXMsg::SliderChange(universe, channel, value) => {
                info!("Changed slider [{}]:{} to {}", universe, channel, value);
                match self.dmx.get_universe(universe) {
                    Some(u) => {
                        u.set_channel(channel, value);
                    }
                    None => error!("given Universe {} is not configured!", universe),
                }
            }
            ConMXMsg::Grid => info!("Grid change called!"),
            ConMXMsg::RandomChange => info!("RandomChange called!"),
        }
        Command::none()
    }

    fn view(&mut self) -> Element<ConMXMsg>{
        let but = Button::new(
                &mut self.bstate, 
                Text::new("This should be a button")
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .vertical_alignment(VerticalAlignment::Center)
                    .horizontal_alignment(HorizontalAlignment::Center))
            .on_press(ConMXMsg::ButtonPressed)
            .width(Length::Fill)
            .style(style::Theme);

        let img = Svg::from_path(format!("{}/assets/pie_chart.svg", env!("CARGO_MANIFEST_DIR")))
            .width(Length::Fill)
            .height(Length::Fill);

        let img_but = Button::new(
            &mut self.imgbstate,
            Svg::from_path(format!("{}/assets/pie_chart.svg", env!("CARGO_MANIFEST_DIR")))
                .height(Length::Units(40))
        )
            .on_press(ConMXMsg::RandomChange)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::Theme);

        let grid = Grid::new();

        let subcont = Row::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(Align::Center)
            .push(self.grid.view()
                .map(move |msg| ConMXMsg::Grid));
        /*
            .push(but)
            .push(img)
            .push(img_but);
        */

        let menu_line = Row::new()
            .height(Length::Units(100))
            .width(Length::Fill)
            .push(
                Text::new("This is some text")
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .size(30)
            );


        let bottom_tab_line = Row::new()
            .height(Length::Units(50))
            .width(Length::Fill)
            .align_items(Align::Center)
            .push(
                Text::new("This is some text")
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center)
            );

        let view_split = Column::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .push(menu_line)
            .push(subcont)
            .push(bottom_tab_line);
        

        Container::new(view_split)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
