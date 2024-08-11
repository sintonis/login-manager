use alloc::rc::Rc;
use iced::{Command, Element, Subscription};
use iced::futures::TryFutureExt;
use iced::widget::{Button, Column, container, Scrollable, text};
use crate::gui::views::Action;
use crate::gui::views::user::UserViewMessage;
use crate::utils::user::User;

#[derive(Debug, Default)]
pub struct UserListPanel {
}


impl UserListPanel {
    pub fn update(&mut self, message: UserViewMessage) -> Command<UserViewMessage> {
        Command::none()
    }

    pub fn view(&self, users: &Vec<User>) -> Element<UserViewMessage> {
        let title = text("Select").size(40);

        let user_list = users.iter().fold(Column::new().spacing(10), |col, user| {
            col.push(Button::new(text(&user.name))
                .on_press(UserViewMessage::UserSelected(user.clone())))
        });

        Column::new()
            .spacing(20)
            .push(title)
            .push(Scrollable::new(user_list)).into()
    }
}
