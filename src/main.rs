use argh::FromArgs;
use iced::{
    alignment::Horizontal,
    executor,
    widget::{Button, Column, Container, Row, Text},
    window, Alignment, Application, Command, Element, Font, Length, Settings,
    Theme,
};
use std::process::Command as StdCommand;

#[cfg(unix)]
use std::os::unix::process::CommandExt;

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

fn format_command(x: &str, y: Option<&str>) -> String {
    if let Some(c) = y {
        x.replace("{}", c)
    } else {
        x.into()
    }
}

#[cfg(windows)]
fn run_command(command: String) -> Command<Message> {
    Command::batch([
        window::close(window::Id::MAIN),
        Command::perform(
            async move { StdCommand::new("cmd").arg("/c").arg(command).spawn() },
            |_| unreachable!(),
        ),
    ])
}

#[cfg(unix)]
fn run_command(command: String) -> Command<Message> {
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

impl Application for State {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Args;

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        let mut profiles = Vec::new();
        let mut iter = flags.profiles.into_iter();
        while let (Some(x), Some(y)) = (iter.next(), iter.next()) {
            profiles.push((x, y));
        }
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
        let (_, value) = self.profiles[message.choice].clone();
        let command = format_command(&value, self.command.as_deref());
        run_command(command)
    }

    fn view(&self) -> Element<Message> {
        let content =
            Column::with_children(self.profiles.iter().enumerate().map(
                |(index, (name, value))| {
                    let command = format_command(
                        value,
                        self.command.as_ref().map(|_| "%command%"),
                    );
                    let words = Row::new()
                        .push(
                            Text::new(name)
                                .horizontal_alignment(Horizontal::Left),
                        )
                        .push(
                            Text::new(command)
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
