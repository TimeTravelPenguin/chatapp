use http::uri::Authority;
use iced::{
    self, Length, Theme,
    widget::{Text, button, container, text_editor},
};

use crate::client::screens::chatroom::{ChatroomMessage, ChatroomScreen};

#[derive(Debug, Clone)]
pub struct ClientApp {
    pub theme: Theme,
    server: Authority,
    screen: Screen,
}

#[derive(Debug, Clone)]
pub enum ConnectionMessage {
    Connecting,
    Connected,
    Failed,
}

#[derive(Debug, Clone)]
pub enum ClientMessage {
    Connection(ConnectionMessage),
    Chatroom(ChatroomMessage),
}

#[derive(Debug, Clone)]
pub enum Screen {
    Startup,
    Login,
    Signup,
    Chatroom(ChatroomScreen),
}

impl ClientApp {
    pub fn new(server: Authority) -> Self {
        Self {
            theme: Theme::CatppuccinMocha,
            screen: Screen::Startup,
            server,
        }
    }

    pub fn update(&mut self, message: ClientMessage) {
        match message {
            ClientMessage::Connection(_message) => {
                // temp: just transition to chatroom screen after "connecting"
                self.screen = Screen::Chatroom(ChatroomScreen::new());
            }
            ClientMessage::Chatroom(message) => {
                if let Screen::Chatroom(chatroom_screen) = &mut self.screen {
                    chatroom_screen.update(message)
                }
            }
        }
    }

    pub fn view(&self) -> iced::Element<'_, ClientMessage> {
        let screen = match &self.screen {
            Screen::Startup => button("Connect")
                .on_press(ClientMessage::Connection(ConnectionMessage::Connecting))
                .into(),
            Screen::Login => Text::new("Login screen (not implemented)").into(),
            Screen::Signup => Text::new("Signup screen (not implemented)").into(),
            Screen::Chatroom(chatroom_screen) => {
                chatroom_screen.view().map(ClientMessage::Chatroom)
            }
        };

        container(screen)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .style(container::rounded_box)
            .into()
    }
}
