use iced::{Command, Element};
use iced::widget::{Column, container, text};
use crate::gui::views::Action;
use crate::gui::views::user_select::UserSelectViewMessage;
use crate::utils::user::User;

#[derive(Debug)]
pub struct UserLoginPanel {
    pub user: User
}


impl UserLoginPanel {
    pub fn new(user: User) -> Self {
        Self { user }
    }
    
    pub fn update(&mut self, message: UserSelectViewMessage) -> Command<UserSelectViewMessage> {
        Command::none()
    }

    pub fn view(&self) -> Element<UserSelectViewMessage> {
        let content = Column::new()
            .spacing(20)
            .push(text("Login"))
            .push(text(&self.user.name));
        
        container(content).padding(20).into()
    }
}
