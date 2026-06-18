use iced::{
    Length,
    alignment::{Horizontal, Vertical},
    padding,
    widget::{button, column, container, row, text_input},
};

use crate::client::screens::field;

#[derive(Debug, Clone)]
pub enum LoginScreenAction {
    None,
    Submit { username: String, password: String },
    Cancel,
}

#[derive(Debug, Clone)]
pub enum LoginScreenMessage {
    SubmitPressed,
    CancelPressed,
    UsernameChanged(String),
    PasswordChanged(String),
}

#[derive(Debug, Clone, Default)]
pub struct LoginScreen {
    username: String,
    password: String,
}

impl LoginScreen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, message: LoginScreenMessage) -> LoginScreenAction {
        match message {
            LoginScreenMessage::SubmitPressed => LoginScreenAction::Submit {
                username: self.username.clone(),
                password: self.password.clone(),
            },
            LoginScreenMessage::CancelPressed => LoginScreenAction::Cancel,
            LoginScreenMessage::UsernameChanged(username) => {
                self.username = username;
                LoginScreenAction::None
            }
            LoginScreenMessage::PasswordChanged(password) => {
                self.password = password;
                LoginScreenAction::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<'_, LoginScreenMessage> {
        let form = column![
            field(
                "Username:",
                text_input("Username", &self.username)
                    .on_input(LoginScreenMessage::UsernameChanged),
            ),
            field(
                "Password:",
                text_input("Password", &self.password)
                    .secure(true)
                    .on_input(LoginScreenMessage::PasswordChanged),
            ),
            row![
                button("Login")
                    .on_press(LoginScreenMessage::SubmitPressed)
                    .style(button::primary),
                button("Cancel")
                    .on_press(LoginScreenMessage::CancelPressed)
                    .style(button::secondary),
            ]
            .padding(padding::top(10))
            .spacing(20),
        ]
        .spacing(12)
        .align_x(Horizontal::Center);

        container(form)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into()
    }
}
