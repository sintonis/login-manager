use iced::{Command, Element, Subscription};
use iced::widget::{Button, container, text};
use crate::gui::views::{Action, View};

#[derive(Debug, Default)]
pub struct UserCreateView {
}

#[derive(Debug, Clone)]
pub enum UserCreateViewMessage {
    Action(Action),
    SwitchButtonPressed,
}

impl UserCreateView {
    pub fn update(&self, message: UserCreateViewMessage) -> Command<UserCreateViewMessage> {
        match message {
            UserCreateViewMessage::SwitchButtonPressed => {
                // When the button is pressed, switch to the UserSelect view
                Command::perform(async { UserCreateViewMessage::Action(Action::SwitchView(View::UserSelect)) }, |msg| msg)
            }
            UserCreateViewMessage::Action(action) => {
                // Handle actions here, for now we just return none
                match action {
                    Action::SwitchView(view) => {
                        // Handle view switch logic here, possibly in your main application logic
                        println!("Switching to view: {:?}", view);
                    }
                }
                Command::none()
            }
        }
    }

    pub fn view(&self) -> Element<UserCreateViewMessage> {
        let button = Button::new(text("Switch to User Select"))
            .on_press(UserCreateViewMessage::SwitchButtonPressed);

        container(button)
            .padding(20)
            .into()
    }

    pub fn subscription(&self) -> Subscription<UserCreateViewMessage> {
        Subscription::none()
    }
}