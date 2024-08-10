use iced::{Command, Element, Subscription};
use iced::widget::{container, text};
use crate::gui::views::Action;
use crate::utils::user::User;

#[derive(Debug, Default)]
pub struct UserListPanel {}

#[derive(Debug, Clone)]
pub enum UserListPanelMessage {
    Action(Action)
}


impl UserListPanel {
    pub fn update(&mut self, message: UserListPanelMessage) -> Command<UserListPanelMessage> {
        Command::none()
    }

    pub fn view(&self, users: User) -> Element<UserListPanelMessage> {
        container(text("List")).padding(20).into()
    }
}
