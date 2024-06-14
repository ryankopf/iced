use iced::executor;
use iced::multi_window::{self, Application};
use iced::widget::{
    button, center, column, container, horizontal_space, scrollable, text,
    text_input,
};
use iced::window;
use iced::{
    Alignment, Element, Length, Settings, Subscription, Task, Theme, Vector,
};

use std::collections::BTreeMap;

fn main() -> iced::Result {
    Example::run(Settings::default())
}

#[derive(Default)]
struct Example {
    windows: BTreeMap<window::Id, Window>,
}

#[derive(Debug)]
struct Window {
    title: String,
    scale_input: String,
    current_scale: f64,
    theme: Theme,
    input_id: iced::widget::text_input::Id,
}

#[derive(Debug, Clone)]
enum Message {
    OpenWindow,
    WindowOpened(window::Id),
    WindowClosed(window::Id),
    ScaleInputChanged(window::Id, String),
    ScaleChanged(window::Id, String),
    TitleChanged(window::Id, String),
}

impl multi_window::Application for Example {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Task<Message>) {
        (
            Example {
                windows: BTreeMap::from([(window::Id::MAIN, Window::new(1))]),
            },
            Task::none(),
        )
    }

    fn title(&self, window: window::Id) -> String {
        self.windows
            .get(&window)
            .map(|window| window.title.clone())
            .unwrap_or_default()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenWindow => {
                let Some(last_window) = self.windows.keys().last() else {
                    return Task::none();
                };

                window::fetch_position(*last_window)
                    .then(|last_position| {
                        let position = last_position.map_or(
                            window::Position::Default,
                            |last_position| {
                                window::Position::Specific(
                                    last_position + Vector::new(20.0, 20.0),
                                )
                            },
                        );

                        window::open(window::Settings {
                            position,
                            ..window::Settings::default()
                        })
                    })
                    .map(Message::WindowOpened)
            }
            Message::WindowOpened(id) => {
                self.windows.insert(id, Window::new(self.windows.len() + 1));

                if let Some(window) = self.windows.get(&id) {
                    text_input::focus(window.input_id.clone())
                } else {
                    Task::none()
                }
            }
            Message::WindowClosed(id) => {
                self.windows.remove(&id);

                Task::none()
            }
            Message::ScaleInputChanged(id, scale) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.scale_input = scale;
                }

                Task::none()
            }
            Message::ScaleChanged(id, scale) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.current_scale = scale
                        .parse::<f64>()
                        .unwrap_or(window.current_scale)
                        .clamp(0.5, 5.0);
                }

                Task::none()
            }
            Message::TitleChanged(id, title) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.title = title;
                }

                Task::none()
            }
        }
    }

    fn view(&self, window_id: window::Id) -> Element<Message> {
        if let Some(window) = self.windows.get(&window_id) {
            center(window.view(window_id)).into()
        } else {
            horizontal_space().into()
        }
    }

    fn theme(&self, window: window::Id) -> Theme {
        if let Some(window) = self.windows.get(&window) {
            window.theme.clone()
        } else {
            Theme::default()
        }
    }

    fn scale_factor(&self, window: window::Id) -> f64 {
        self.windows
            .get(&window)
            .map(|window| window.current_scale)
            .unwrap_or(1.0)
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        window::closings().map(Message::WindowClosed)
    }
}

impl Window {
    fn new(count: usize) -> Self {
        Self {
            title: format!("Window_{}", count),
            scale_input: "1.0".to_string(),
            current_scale: 1.0,
            theme: if count % 2 == 0 {
                Theme::Light
            } else {
                Theme::Dark
            },
            input_id: text_input::Id::unique(),
        }
    }

    fn view(&self, id: window::Id) -> Element<Message> {
        let scale_input = column![
            text("Window scale factor:"),
            text_input("Window Scale", &self.scale_input)
                .on_input(move |msg| { Message::ScaleInputChanged(id, msg) })
                .on_submit(Message::ScaleChanged(
                    id,
                    self.scale_input.to_string()
                ))
        ];

        let title_input = column![
            text("Window title:"),
            text_input("Window Title", &self.title)
                .on_input(move |msg| { Message::TitleChanged(id, msg) })
                .id(self.input_id.clone())
        ];

        let new_window_button =
            button(text("New Window")).on_press(Message::OpenWindow);

        let content = scrollable(
            column![scale_input, title_input, new_window_button]
                .spacing(50)
                .width(Length::Fill)
                .align_items(Alignment::Center),
        );

        container(content).center_x(200).into()
    }
}
