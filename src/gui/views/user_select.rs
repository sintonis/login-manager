use iced::{Command, Element, Subscription};
use iced::widget::{container, text, Column, Button};
use crate::gui::panels::user_list::UserListPanel;
use crate::gui::views::{Action, View};
use crate::utils::user::User;

#[derive(Debug, Default)]
pub struct UserSelectView {
    user_list_panel: UserListPanel
}

#[derive(Debug, Clone)]
pub enum UserSelectViewMessage {
    Action(Action),
    SwitchButtonPressed,
}

impl UserSelectView {
    pub fn update(&mut self, message: UserSelectViewMessage) -> Command<UserSelectViewMessage> {
        match message {
            UserSelectViewMessage::SwitchButtonPressed => {
                // Send a command to switch to the UserCreate view
                Command::perform(async { UserSelectViewMessage::Action(Action::SwitchView(View::UserCreate))}, |msg| msg)
            }
            UserSelectViewMessage::Action(_action) => {
                Command::none()
            }
        }
    }

    pub fn view(&self, users: &Vec<User>) -> Element<UserSelectViewMessage> {
        let title = text("Select").size(40);

        let button_content = text("Switch to Create User");
        let button = Button::new(button_content)
            .on_press(UserSelectViewMessage::SwitchButtonPressed);

        // Arrange title and button in a column
        let content = Column::new()
            .spacing(20)
            .push(title)
            .push(button);

        container(content)
            .padding(20)
            .into()
    }

    pub fn subscription(&self) -> Subscription<UserSelectViewMessage> {
        Subscription::none()
    }
}
