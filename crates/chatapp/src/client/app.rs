use http::uri::Authority;
use iced::{
    self, Event, Length, Task, Theme, event, keyboard,
    widget::{Text, button, container, text_editor},
};
use tracing::info;

use crate::client::screens::{
    chatroom::{ChatroomMessage, ChatroomScreen},
    login::{LoginScreen, LoginScreenAction, LoginScreenMessage},
    signup::{SignupScreen, SignupScreenAction, SignupScreenMessage},
    startup::{StartupScreen, StartupScreenAction, StartupScreenMessage},
};

#[derive(Debug, Clone)]
pub struct ClientApp {
    pub theme: Theme,
    server: Authority,
    screen: Screen,
}

#[derive(Debug, Clone)]
pub enum ClientMessage {
    Startup(StartupScreenMessage),
    Login(LoginScreenMessage),
    Signup(SignupScreenMessage),
    Chatroom(ChatroomMessage),
    FocusNext,
    FocusPrevious,
}

#[derive(Debug, Clone)]
pub enum Screen {
    Startup(StartupScreen),
    Login(LoginScreen),
    Signup(SignupScreen),
    Chatroom(ChatroomScreen),
}

impl ClientApp {
    pub fn new(server: Authority) -> Self {
        Self {
            theme: Theme::CatppuccinMocha,
            screen: Screen::Startup(StartupScreen::new()),
            server,
        }
    }

    pub fn update(&mut self, message: ClientMessage) -> Task<ClientMessage> {
        match message {
            ClientMessage::Startup(message) => {
                if let Screen::Startup(startup) = &mut self.screen {
                    match startup.update(message) {
                        // StartupScreenAction::None => Task::none(),
                        StartupScreenAction::Login => {
                            info!("Switching to login screen");
                            self.screen = Screen::Login(LoginScreen::new());
                            Task::none()
                        }

                        StartupScreenAction::Signup => {
                            info!("Switching to signup screen");
                            self.screen = Screen::Signup(SignupScreen::new());
                            Task::none()
                        }
                    }
                } else {
                    Task::none()
                }
            }

            ClientMessage::Login(message) => {
                if let Screen::Login(login) = &mut self.screen {
                    match login.update(message) {
                        LoginScreenAction::None => Task::none(),
                        LoginScreenAction::Submit { .. } => {
                            info!("Submitting login form");
                            self.screen = Screen::Chatroom(ChatroomScreen::new());
                            // TODO: Create a task to login.
                            Task::none()
                        }
                        LoginScreenAction::Cancel => {
                            info!("Cancelling login, returning to startup screen");
                            self.screen = Screen::Startup(StartupScreen::new());
                            Task::none()
                        }
                    }
                } else {
                    Task::none()
                }
            }

            ClientMessage::Signup(message) => {
                if let Screen::Signup(signup) = &mut self.screen {
                    match signup.update(message) {
                        SignupScreenAction::None => Task::none(),
                        SignupScreenAction::Submit { .. } => {
                            info!("Submitting signup form");
                            self.screen = Screen::Chatroom(ChatroomScreen::new());
                            // TODO: Create a task to signup.
                            Task::none()
                        }
                        SignupScreenAction::Cancel => {
                            info!("Cancelling signup, returning to startup screen");
                            self.screen = Screen::Startup(StartupScreen::new());
                            Task::none()
                        }
                    }
                } else {
                    Task::none()
                }
            }

            ClientMessage::Chatroom(message) => {
                if let Screen::Chatroom(chatroom) = &mut self.screen {
                    chatroom.update(message);
                    Task::none()
                } else {
                    Task::none()
                }
            }

            ClientMessage::FocusNext => iced::widget::operation::focus_next(),
            ClientMessage::FocusPrevious => iced::widget::operation::focus_previous(),
        }
    }

    pub fn view(&self) -> iced::Element<'_, ClientMessage> {
        let screen = match &self.screen {
            Screen::Startup(screen) => screen.view().map(ClientMessage::Startup),
            Screen::Login(screen) => screen.view().map(ClientMessage::Login),
            Screen::Signup(screen) => screen.view().map(ClientMessage::Signup),
            Screen::Chatroom(screen) => screen.view().map(ClientMessage::Chatroom),
        };

        container(screen)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .style(container::rounded_box)
            .into()
    }

    pub fn subscription(&self) -> iced::Subscription<ClientMessage> {
        event::listen_with(|event, _status, _window| match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Tab),
                modifiers,
                ..
            }) if modifiers.shift() => Some(ClientMessage::FocusPrevious),

            iced::Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Tab),
                modifiers,
                ..
            }) if !modifiers.shift() => Some(ClientMessage::FocusNext),

            _ => None,
        })
    }
}
