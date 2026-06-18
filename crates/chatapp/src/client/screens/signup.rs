use iced::{
    Length,
    alignment::{Horizontal, Vertical},
    padding,
    widget::{button, column, container, row, text_input},
};

use crate::client::screens::field;

#[derive(Debug, Clone)]
pub enum SignupScreenAction {
    None,
    Submit {
        username: String,
        password: String,
        email: String,
    },
    Cancel,
}

#[derive(Debug, Clone)]
pub enum SignupScreenMessage {
    SubmitPressed,
    CancelPressed,
    UsernameChanged(String),
    PasswordChanged(String),
    EmailChanged(String),
}

#[derive(Debug, Clone, Default)]
pub struct SignupScreen {
    username: String,
    password: String,
    email: String,
}

impl SignupScreen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, message: SignupScreenMessage) -> SignupScreenAction {
        match message {
            SignupScreenMessage::SubmitPressed => SignupScreenAction::Submit {
                username: self.username.clone(),
                password: self.password.clone(),
                email: self.email.clone(),
            },
            SignupScreenMessage::CancelPressed => SignupScreenAction::Cancel,
            SignupScreenMessage::UsernameChanged(username) => {
                self.username = username;
                SignupScreenAction::None
            }
            SignupScreenMessage::PasswordChanged(password) => {
                self.password = password;
                SignupScreenAction::None
            }
            SignupScreenMessage::EmailChanged(email) => {
                self.email = email;
                SignupScreenAction::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<'_, SignupScreenMessage> {
        let form = column![
            field(
                "Username:",
                text_input("Username", &self.username)
                    .on_input(SignupScreenMessage::UsernameChanged),
            ),
            field(
                "Email:",
                text_input("Email", &self.email).on_input(SignupScreenMessage::EmailChanged),
            ),
            field(
                "Password:",
                text_input("Password", &self.password)
                    .secure(true)
                    .on_input(SignupScreenMessage::PasswordChanged),
            ),
            row![
                button("Signup")
                    .on_press(SignupScreenMessage::SubmitPressed)
                    .style(button::primary),
                button("Cancel")
                    .on_press(SignupScreenMessage::CancelPressed)
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
