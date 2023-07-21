use std::{ops::AddAssign, time::Duration};

use iced::{
    widget::{button, container, row, text, Column, Row},
    Element,
};
use iced_aw::{graphics::icons, TabLabel};

use crate::{
    stopwatch::{CounterMessage, State},
    Message, Tab,
};

#[derive(Clone, Debug)]
pub enum TimerMesssage {
    DuractionSelectPressed(usize),
}
#[derive(Default)]
pub struct Timer {
    duration_select: DurationSelect,
}
pub struct DurationSelect {
    inner: Vec<Duration>,
}
impl Default for DurationSelect {
    fn default() -> Self {
        Self {
            inner: (1..)
                .step_by(5)
                .take(8)
                .map(|x| Duration::from_secs(x * 60))
                .collect(),
        }
    }
}
impl DurationSelect {
    pub fn new(list: Vec<Duration>) -> Self {
        Self { inner: list }
    }
    pub fn update(&mut self, message: TimerMesssage) {
        match message {
            TimerMesssage::DuractionSelectPressed(idx) => self.inner[idx],
        };
    }
}
impl Timer {
    pub fn new(duration_select: DurationSelect) -> Self {
        Self { duration_select }
    }
}
impl Tab for Timer {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Timer")
    }

    fn tab_label(&self) -> TabLabel {
        //TabLabel::Text(self.title())
        TabLabel::IconText(icons::Icon::HourglassSplit.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let mut idx = 0;
        let buttons = self
            .duration_select
            .inner
            .chunks(4)
            .fold(Column::new(), |acc, new_col| {
                let new_col = new_col.iter().fold(row![], |acc_col, dur| {
                    acc_col.push(
                        button(text(format!("{} min", dur.as_secs() * 60))).on_press(
                            TimerMesssage::DuractionSelectPressed({
                                let tmp = idx;
                                idx += 1;
                                tmp
                            }),
                        ),
                    )
                });
                acc.push(new_col)
            });
        let content: Element<'_, TimerMesssage> = buttons.into();
        content.map(Message::Timer)
    }
}
