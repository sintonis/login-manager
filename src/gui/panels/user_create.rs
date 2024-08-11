use alloc::rc::Rc;
use iced::{Command, Element};
use iced::widget::{Column, container, text};
use crate::gui::views::user::UserViewMessage;
use crate::utils::user::User;

#[derive(Debug, Default)]
pub struct UserCreatePanel {
}


impl UserCreatePanel {
    pub fn update(&mut self, message: UserViewMessage) -> Command<UserViewMessage> {
        Command::none()
    }

    pub fn view(&self, users: Vec<User>) -> Element<UserViewMessage> {
        let content = Column::new()
            .spacing(20)
            .push(text("Create"));
        
        container(content).padding(20).into()


    }
}
