use iced::executor;
use iced::widget::{button, column, container, row};
use iced::window;
use iced::{Alignment, Application, Command, Element, Length, Settings, Theme};
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
        let content = {
            column(
                self.profiles
                    .iter()
                    .enumerate()
                    .map(|(index, (name, value))| {
                        button(row![name.as_str(), value.as_str(),])
                            .on_press(Message { choice: index })
                            .into()
                    }),
            )
        }
        .spacing(10)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}
