use iced::{Command, Element, Length, Subscription};
use iced::widget::{text, Column, Button, Scrollable, container, Image, Container};
use iced_aw::widgets::Modal;
use crate::gui::panels::user_list::UserListPanel;
use crate::gui::panels::user_login::UserLoginPanel;
use crate::gui::views::Action;
use crate::utils::user::User;

#[derive(Debug, Default)]
pub struct UserSelectView {
    user_list_panel: UserListPanel,
    user_login_panel: Option<UserLoginPanel>, // Track the current login panel
}

#[derive(Debug, Clone)]
pub enum UserSelectViewMessage {
    Action(Action),
    UserSelected(User),  // Handle user selection
    BackButtonPressed,   // Handle going back to user list
}

impl UserSelectView {
    pub fn new(user_list_panel: UserListPanel) -> Self {
        Self {
            user_list_panel,
            user_login_panel: None, // Start by showing the user list
        }
    }

    pub fn update(&mut self, message: UserSelectViewMessage) -> Command<UserSelectViewMessage> {
        match message {
            UserSelectViewMessage::UserSelected(user) => {
                // When a user is selected, switch to the login panel
                self.user_login_panel = Some(UserLoginPanel::new(user));
                Command::none()
            }
            UserSelectViewMessage::BackButtonPressed => {
                // Go back to the user list
                self.user_login_panel = None;
                Command::none()
            }
            _ => Command::none()
        }
    }

    pub fn view(&self, users: &Vec<User>) -> Element<UserSelectViewMessage> {
        let image = Image::new("assets/images/background.png")
            .width(Length::Fill)
            .height(Length::Fill);

        let background = Container::new(image)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        if let Some(user_login_panel) = &self.user_login_panel {
            // Show the login panel if a user is selected
            let back_button = Button::new(text("Back"))
                .on_press(UserSelectViewMessage::BackButtonPressed);

            let content = Column::new()
                .spacing(20)
                .push(user_login_panel.view())
                .push(back_button);

            Modal::new(background, container(content).padding(20).into()).into()
        } else {
            // Show the user list
            let title = text("Select").size(40);

            let user_list = users.iter().fold(Column::new().spacing(10), |col, user| {
                col.push(Button::new(text(&user.name))
                    .on_press(UserSelectViewMessage::UserSelected(user.clone())))
            });

            let content = Column::new()
                .spacing(20)
                .push(title)
                .push(Scrollable::new(user_list));

            Modal::new(background, container(content).padding(20).into()).into()
        }
    }

    pub fn subscription(&self) -> Subscription<UserSelectViewMessage> {
        Subscription::none()
    }
}
