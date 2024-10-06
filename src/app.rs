use super::Pom;

use std::time::Duration;

use iced::{
    executor, time,
    widget::{button, column, container, text},
    Application, Command, Element, Length, Padding, Subscription, Theme,
};

use iced::theme::Button as ButtonTheme;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Toggle,
    Tick,
}

pub struct App {
    task_name: String,
    time: i32,
    mem: i32,
    is_running: bool,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Pom;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let time = flags.duration_minutes * 60;
        (
            App {
                task_name: flags.name.to_uppercase(),
                time,
                mem: time,
                is_running: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Pomodoro Timer".to_string()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        if self.is_running {
            time::every(Duration::from_secs(1)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Toggle => {
                self.is_running = !self.is_running;
            }
            Message::Tick => {
                if self.is_running {
                    if self.time > 0 {
                        self.time -= 1;
                    } else {
                        self.is_running = false;
                        self.time = self.mem;
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let button_label = if self.is_running { "Pause" } else { "Start" };

        let hours = self.time / 3600;
        let minutes = (self.time % 3600) / 60;
        let seconds = self.time % 60;

        let time_formatted = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);

        container(column![
            text(self.task_name.clone()),
            text(time_formatted),
            button(button_label)
                .padding(Padding::from([5, 0, 0, 30]))
                .width(100)
                .height(30)
                .style(ButtonTheme::custom(style::CustomButton))
                .on_press(Message::Toggle),
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

mod style {
    use iced::widget::button;
    use iced::{Background, BorderRadius, Color, Vector};

    pub struct CustomButton;

    impl button::StyleSheet for CustomButton {
        type Style = iced::Theme;

        fn active(&self, _style: &Self::Style) -> button::Appearance {
            button::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.2, 0.5, 0.8))),
                border_radius: BorderRadius::from(20.0),
                text_color: Color::WHITE,
                ..button::Appearance::default()
            }
        }

        fn hovered(&self, style: &Self::Style) -> button::Appearance {
            button::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.3, 0.6, 0.9))),
                shadow_offset: Vector::new(1.0, 2.0),
                ..self.active(style)
            }
        }
    }
}
