use super::Pom;
use iced::theme::Button as ButtonTheme;
use iced::{
    executor, time,
    widget::{button, container, text, Column},
    Application, Command, Element, Length, Subscription, Theme,
};
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Toggle,
    Tick,
    Select(usize),
}

pub struct App {
    tasks: Vec<Pom>,
    time: Option<i32>,
    mem: Option<i32>,
    is_running: bool,
    selected_task: Option<usize>,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Vec<Pom>;

    fn new(tasks: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            App {
                tasks,
                time: None,
                mem: None,
                is_running: false,
                selected_task: None,
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
            Message::Select(id) => {
                self.selected_task = Some(id);
                if let Some(task) = self.tasks.get(id) {
                    let time = Some(task.duration_minutes * 60);
                    self.time = time;
                    self.mem = time;
                    self.is_running = false;
                }
            }
            Message::Toggle => {
                if self.time.is_some() {
                    self.is_running = !self.is_running;
                }
            }
            Message::Tick => {
                if let Some(time) = self.time {
                    if time > 0 {
                        self.time = Some(time - 1);
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
        let timer_text = if let Some(time) = self.time {
            let minutes = time / 60;
            let seconds = time % 60;
            format!("{:02}:{:02}", minutes, seconds)
        } else {
            "00:00".to_string()
        };

        let timer_display = text(timer_text).size(40);

        let start_stop_button = button(text(if self.is_running { "STOP" } else { "START" }))
            .style(ButtonTheme::custom(style::CustomButton))
            .on_press(Message::Toggle);

        let task_buttons = Column::with_children(
            self.tasks
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let is_selected = self.selected_task == Some(i);
                    button(text(item.name.to_uppercase()))
                        .width(200)
                        .style(if is_selected {
                            ButtonTheme::Primary
                        } else {
                            ButtonTheme::Secondary
                        })
                        .on_press(Message::Select(i))
                        .into()
                })
                .collect(),
        )
        .spacing(10);

        let content = Column::new()
            .push(timer_display)
            .push(task_buttons)
            .push(start_stop_button)
            .spacing(20)
            .align_items(iced::Alignment::Center);

        container(content)
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
