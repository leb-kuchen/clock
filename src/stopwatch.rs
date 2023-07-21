use std::time::{Duration, Instant};

use iced::{
    widget::{text, Button, Column, Container, Row, Text},
    Alignment, Element,
};
use iced_aw::{graphics::icons, tab_bar::TabLabel};

use crate::{Message, Tab};

#[derive(Debug, Clone)]
pub enum CounterMessage {
    Toggle,
    Reset,
    Tick(Instant),
}
#[derive(Default, Debug, Clone)]
pub enum State {
    #[default]
    Idle,
    Ticking {
        last_tick: Instant,
    },
}

#[derive(Default, Clone)]
pub struct Stopwatch {
    value: i32,
    duration: Duration,
    pub state: State,
}

impl Stopwatch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, message: CounterMessage) {
        match message {
            CounterMessage::Toggle => match self.state {
                State::Idle => {
                    self.state = State::Ticking {
                        last_tick: Instant::now(),
                    }
                }
                State::Ticking { last_tick: _ } => self.state = State::Idle,
            },
            CounterMessage::Reset => self.duration = Duration::ZERO,
            CounterMessage::Tick(now) => {
                if let State::Ticking { last_tick } = &mut self.state {
                    self.duration += now - *last_tick;
                    *last_tick = now
                }
            }
        }
    }
}

impl Tab for Stopwatch {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Stopuhr")
    }

    fn tab_label(&self) -> TabLabel {
        //TabLabel::Text(self.title())
        TabLabel::IconText(icons::Icon::Stopwatch.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let content: Element<'_, CounterMessage> = Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .max_width(600)
                .padding(20)
                .spacing(16)
                .push(Text::new(format!("Count: {}", self.value)).size(32))
                .push(
                    Row::new()
                        .spacing(10)
                        .push(text(format!("{:?}", self.duration)))
                        .push(Button::new(Text::new("Toggle")).on_press(CounterMessage::Toggle))
                        .push(Button::new(Text::new("Reset")).on_press(CounterMessage::Reset)),
                ),
        )
        .into();

        content.map(Message::Counter)
    }
}
