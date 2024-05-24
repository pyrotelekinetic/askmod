use iced::executor;
use iced::widget::{button, column, container, row};
use iced::window;
use iced::{Alignment, Application, Command, Element, Length, Settings, Theme};
use std::os::unix::process::CommandExt;
use std::process::Command as StdCommand;

pub fn main() -> iced::Result {
    State::run(Settings::default())
}

#[derive(Default)]
struct State {
    profiles: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
struct Message {
    choice: usize,
}

impl Application for State {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
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
                    .map(|(name, value)| button(row![name.as_str(), value.as_str(),]).into()),
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
