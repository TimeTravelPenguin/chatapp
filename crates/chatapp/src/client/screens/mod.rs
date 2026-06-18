use iced::{
    Length,
    alignment::{Horizontal, Vertical},
    widget::{TextInput, row, text},
};

pub mod chatroom;
pub mod login;
pub mod signup;
pub mod startup;

const LABEL_WIDTH: Length = Length::Fixed(100.0);
const INPUT_WIDTH: Length = Length::Fixed(280.0);

pub fn field<'a, Message>(
    label: &'static str,
    input: TextInput<'a, Message>,
) -> iced::Element<'a, Message>
where
    Message: Clone + 'a,
{
    row![
        text(label).width(LABEL_WIDTH).align_x(Horizontal::Right),
        input.width(INPUT_WIDTH),
    ]
    .spacing(20)
    .align_y(Vertical::Center)
    .into()
}
