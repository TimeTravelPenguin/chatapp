use iced::{
    Length,
    alignment::{Horizontal, Vertical},
    padding,
    widget::{button, column, container, row, text},
};

#[derive(Debug, Clone)]
pub enum StartupScreenMessage {
    LoginPressed,
    SignupPressed,
}

#[derive(Debug, Clone)]
pub enum StartupScreenAction {
    Login,
    Signup,
}

#[derive(Debug, Clone)]
pub struct StartupScreen;

impl StartupScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, message: StartupScreenMessage) -> StartupScreenAction {
        match message {
            StartupScreenMessage::LoginPressed => StartupScreenAction::Login,
            StartupScreenMessage::SignupPressed => StartupScreenAction::Signup,
        }
    }

    pub fn view(&self) -> iced::Element<'_, StartupScreenMessage> {
        let col = column![welcome_text(), buttons()].spacing(20);

        container(col)
            .height(Length::Fill)
            .width(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into()
    }
}

fn welcome_text() -> iced::Element<'static, StartupScreenMessage> {
    container(text("Welcome to ChatApp!").size(40))
        .width(Length::Fill)
        .padding(padding::bottom(20))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
}

fn buttons() -> iced::Element<'static, StartupScreenMessage> {
    let width = 80;

    let button_row = row![
        button(text("Login").align_x(Horizontal::Center))
            .style(button::primary)
            .on_press(StartupScreenMessage::LoginPressed)
            .width(width),
        button(text("Signup").align_x(Horizontal::Center))
            .style(button::secondary)
            .on_press(StartupScreenMessage::SignupPressed)
            .width(width),
    ]
    .spacing(20);

    container(button_row)
        .width(iced::Length::Fill)
        // .padding(20)
        .align_x(Horizontal::Center)
        .into()
}
