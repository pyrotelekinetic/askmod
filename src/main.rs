use iced::alignment::Horizontal;
use iced::executor;
use iced::widget::{Button, Column, Container, Row, Text};
use iced::window;
use iced::{Alignment, Application, Command, Element, Font, Length, Settings, Theme};
use std::os::unix::process::CommandExt;
use std::process::Command as StdCommand;

#[derive(Default)]
struct State {
    profiles: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
struct Message {
    choice: usize,
}

pub fn main() -> iced::Result {
    let mut args = std::env::args().skip(1);
    let mut profiles = Vec::default();
    while let (Some(name), Some(content)) = (args.next(), args.next()) {
        profiles.push((name, content));
    }
    State::run(Settings {
        flags: State { profiles },
        window: window::Settings {
            position: window::Position::Centered,
            level: window::Level::AlwaysOnTop,
            resizable: false,
            size: iced::Size {
                width: 300.0,
                height: 400.0,
            },
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

impl Application for State {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = State;

    fn new(flags: Self) -> (Self, Command<Message>) {
        (flags, Command::none())
    }

    fn title(&self) -> String {
        String::from("askmod")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        let command = self.profiles[message.choice].1.clone();
        Command::batch([
            window::close(window::Id::MAIN),
            Command::perform(
                async move {
                    StdCommand::new("sh")
                        .arg("-c")
                        .arg(format!("exec {}", command))
                        .exec()
                },
                |_| unreachable!(),
            ),
        ])
    }

    fn view(&self) -> Element<Message> {
        let content = Column::with_children(self.profiles.iter().enumerate().map(
            |(index, (name, value))| {
                let words = Row::new()
                    .push(
                        Text::new(name)
                            .horizontal_alignment(Horizontal::Left)
                            .width(Length::Fill),
                    )
                    .push(
                        Text::new(value)
                            .horizontal_alignment(Horizontal::Right)
                            .width(Length::Fill)
                            .font(Font::MONOSPACE)
                            .size(12),
                    )
                    .align_items(Alignment::Center)
                    .width(Length::Fill);
                Button::new(words)
                    .on_press(Message { choice: index })
                    .into()
            },
        ))
        .spacing(10)
        .align_items(Alignment::Center);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}
