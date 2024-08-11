use iced::{alignment, Application, Command, ContentFit, Element, Font, Length, Subscription};
use iced::widget::{text, Column, Button, container, Image, Container, Text};
use iced_aw::widgets::Modal;
use crate::gui::panels::user_create::UserCreatePanel;
use crate::gui::panels::user_list::UserListPanel;
use crate::gui::panels::user_login::UserLoginPanel;
use crate::gui::views::Action;
use crate::utils::user::User;
use chrono::{Duration, Local};
use iced::font::{Family, Stretch, Style, Weight};

#[derive(Debug, Default)]
pub struct UserView {
    users: Vec<User>,

    user_list_panel: UserListPanel,
    user_create_panel: UserCreatePanel,
    user_login_panel: Option<UserLoginPanel>,

    current_time: String,
}

#[derive(Debug, Clone)]
pub enum UserViewMessage {
    Action(Action),
    UserSelected(User),  // Handle user selection
    BackButtonPressed,   // Handle going back to user list
    Tick,
}

fn get_current_time() -> String {
    Local::now().format("%H:%M").to_string()
}

impl UserView {
    pub fn new() -> Self {
        let mut users = User::get_all();

        Self {
            users,

            user_list_panel: UserListPanel::default(),
            user_login_panel: None, // Start by showing the user list
            user_create_panel: UserCreatePanel::default(),
            current_time: get_current_time(),
        }
    }

    pub fn update(&mut self, message: UserViewMessage) -> Command<UserViewMessage> {
        match message {
            UserViewMessage::UserSelected(user) => {
                // When a user is selected, switch to the login panel
                self.user_login_panel = Some(UserLoginPanel::new(user));
                Command::none()
            }
            UserViewMessage::BackButtonPressed => {
                // Go back to the user list
                self.user_login_panel = None;
                Command::none()
            }
            UserViewMessage::Tick => {
                // Update the current time
                self.current_time = get_current_time();
                Command::none()
            }
            _ => Command::none()
        }
    }

    pub fn view(&self) -> Element<UserViewMessage> {
        let image = Image::new("assets/images/background.png")
            .width(Length::Fill)
            .height(Length::Fill).content_fit(ContentFit::Fill);

        let background = Container::new(image)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(alignment::Horizontal::Right)  // Align horizontally to the right (end)
            .align_y(alignment::Vertical::Bottom);

        let clock = Text::new(&self.current_time)
            .size(25).font(Font {
                family: Family::SansSerif,         
                weight: Weight::Black,         
                stretch: Stretch::Normal,         
                style: Style::Normal,
        });

        let clock_header = Container::new(clock)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Left);

        let body = if let Some(user_login_panel) = &self.user_login_panel {
            let back_button = Button::new(Text::new("Back"))
                .on_press(UserViewMessage::BackButtonPressed);

            Column::new()
                .spacing(20)
                .push(user_login_panel.view())
                .push(back_button)
        } else {
            Column::new().spacing(20)
                .push(self.user_list_panel.view(&self.users))
        };

        let content = Column::new()
            .spacing(10)
            .push(clock_header)
            .push(body);

        Modal::new(background, Container::new(content).width(Length::Fill).height(Length::Fill).padding(10).into()).into()
    }

    pub fn subscription(&self) -> Subscription<UserViewMessage> {
        iced::time::every(std::time::Duration::from_millis(10000)).map(|_| {
            UserViewMessage::Tick
        })
    }
}
