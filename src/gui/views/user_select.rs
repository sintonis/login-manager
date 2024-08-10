use iced::{Command, Element, Subscription};
use iced::widget::{container, text};
use crate::gui::views::Action;

#[derive(Debug, Default)]
pub struct UserSelectView {
}

#[derive(Debug, Clone)]
pub enum UserSelectViewMessage {
    Action(Action)
}


impl UserSelectView {
    pub fn update(&mut self, message: UserSelectViewMessage) -> Command<UserSelectViewMessage> {
        Command::none()
    }

    pub fn view(&self) -> Element<UserSelectViewMessage> {
        container(text("Select")).padding(20).into()
    }

    pub fn subscription(&self) -> Subscription<UserSelectViewMessage> {
        Subscription::none()
    }
}





