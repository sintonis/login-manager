use iced::{Command, Element};
use iced::widget::{Column, container, text};
use crate::gui::views::Action;
use crate::utils::user::User;

#[derive(Debug, Default)]
pub struct UserLoginPanel {}

#[derive(Debug, Clone)]
pub enum UserLoginPanelMessage {
    Action(Action)
}


impl UserLoginPanel {
    pub fn update(&mut self, message: UserLoginPanelMessage) -> Command<UserLoginPanelMessage> {
        Command::none()
    }

    pub fn view(&self, user: User) -> Element<UserLoginPanelMessage> {
        let content = Column::new()
            .spacing(20)
            .push(text("Login"))
            .push(text(user.name));
        
        container(content).padding(20).into()
    }
}
