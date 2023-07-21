use std::time::Duration;

use iced::{
    alignment::{Horizontal, Vertical},
    executor, time,
    widget::{Column, Container, Text},
    Application, Command, Element, Length, Sandbox, Settings, Subscription, Theme,
};
use iced_aw::{TabLabel, Tabs};
mod stopwatch;
use login::{LoginMessage, LoginTab};
use stopwatch::{CounterMessage, Stopwatch};
use timer::{Timer, TimerMesssage};
mod counter;
mod login;
mod timer;

fn main() -> iced::Result {
    App::run(Settings::default())
}
enum Icon {
    User,
    Heart,
    Calc,
    CogAlt,
}

impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        match icon {
            Icon::User => '\u{E800}',
            Icon::Heart => '\u{E801}',
            Icon::Calc => '\u{F597}',
            Icon::CogAlt => '\u{E802}',
        }
    }
}
#[derive(Default)]
struct App {
    active_tab: usize,
    stopwatch: Stopwatch,
    login: LoginTab,
    timer: Timer,
}

#[derive(Debug, Clone)]
enum Message {
    TabSelected(usize),
    Counter(CounterMessage),
    Login(LoginMessage),
    Timer(TimerMesssage),
}

const HEADER_SIZE: u16 = 32;
const TAB_PADDING: u16 = 16;
impl Application for App {
    type Message = Message;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        "hi".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Message> {
        match message {
            Message::TabSelected(selected) => self.active_tab = selected,
            Message::Counter(msg) => self.stopwatch.update(msg),
            Message::Login(msg) => self.login.update(msg),
            Message::Timer(_) => (),
        };
        Command::none()
    }
    fn subscription(&self) -> iced::Subscription<Self::Message> {
        match self.stopwatch.state {
            stopwatch::State::Idle => Subscription::none(),
            stopwatch::State::Ticking { .. } => {
                let _f = CounterMessage::Tick;
                time::every(Duration::from_millis(10))
                    .map(|instant| Message::Counter(CounterMessage::Tick(instant)))
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        Tabs::new(self.active_tab, Message::TabSelected)
            .push(self.login.tab_label(), self.login.view())
            .push(self.stopwatch.tab_label(), self.stopwatch.view())
            .push(self.timer.tab_label(), self.timer.view())
            .into()
    }

    type Executor = executor::Default;

    type Theme = Theme;

    type Flags = ();
}
trait Tab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(Text::new(self.title()).size(HEADER_SIZE))
            .push(self.content());

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(TAB_PADDING)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}
