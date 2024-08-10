use iced::{Command, Element, Subscription};
use iced::widget::{container, text};
use crate::gui::views::Action;

#[derive(Debug, Default)]
pub struct UserLoginView {
}

#[derive(Debug, Clone)]
pub enum UserLoginViewMessage {
    Action(Action)
}


impl UserLoginView {
    pub fn update(&mut self, message: UserLoginViewMessage) -> Command<UserLoginViewMessage> {
        Command::none()
    }

    pub fn view(&self) -> Element<UserLoginViewMessage> {
        container(text("Login")).padding(20).into()
    }

    pub fn subscription(&self) -> Subscription<UserLoginViewMessage> {
        Subscription::none()
    }
}





