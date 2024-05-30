use argh::FromArgs;
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
    name: Option<String>,
    command: Option<String>,
}

#[derive(Debug, Clone)]
struct Message {
    choice: usize,
}

#[derive(FromArgs, Default)]
/// Launch a mod profile
struct Args {
    /// name to display in title
    #[argh(option, short = 'n')]
    name: Option<String>,

    /// command to wrap
    #[argh(option, short = 'c')]
    command: Option<String>,

    /// profile defs
    #[argh(positional, greedy)]
    profiles: Vec<String>,
}

pub fn main() -> iced::Result {
    let args: Args = argh::from_env();
    State::run(Settings {
        flags: args,
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
    type Flags = Args;

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        let profiles = flags
            .profiles
            .chunks(2)
            .map(|c| match c {
                [x, y] => (x.clone(), y.clone()),
                _ => unreachable!(),
            })
            .collect();
        (
            State {
                profiles,
                command: flags.command,
                name: flags.name,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        self.name.clone().unwrap_or(String::from("askmod"))
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        let (_, command) = self.profiles[message.choice].clone();
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
