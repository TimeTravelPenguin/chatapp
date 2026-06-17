use iced::{
    Length,
    widget::{container, text_editor},
};

#[derive(Debug, Clone)]
pub enum ChatroomMessage {
    ChatInput(text_editor::Action),
}

#[derive(Debug, Clone)]
pub struct ChatroomScreen {
    content: text_editor::Content,
}

impl ChatroomScreen {
    pub fn new() -> Self {
        Self {
            content: text_editor::Content::new(),
        }
    }

    pub fn update(&mut self, message: ChatroomMessage) {
        match message {
            ChatroomMessage::ChatInput(action) => self.content.perform(action),
        }
    }

    pub fn view(&self) -> iced::Element<'_, ChatroomMessage> {
        let editor = text_editor(&self.content)
            .placeholder("Type your message here...")
            .height(100)
            .on_action(ChatroomMessage::ChatInput);

        container(editor)
            .width(Length::Fill)
            .height(Length::Shrink)
            .padding(10)
            .style(container::rounded_box)
            .into()
    }
}
