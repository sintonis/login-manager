mod panels;
mod views;

use iced::{Application, Command, Element, Font, Pixels, Settings, Size, Subscription, Theme};
use crate::config::Config;
use crate::gui::views::{Action, View};
use crate::gui::views::splash_screen::{SplashScreenView, SplashScreenViewMessage};
use crate::gui::views::user_create::{UserCreateView, UserCreateViewMessage};
use crate::gui::views::user_select::{UserSelectView, UserSelectViewMessage};
use crate::utils::user::User;

pub fn run(config: Config) -> Result<(), ()> {
    Greeter::run(settings(config)).map_err(|_| ())
}

#[derive(Debug)]
pub struct Greeter {
    pub view: View,

    pub user_select_view: UserSelectView,
    pub user_create_view: UserCreateView,
    pub splash_screen_view: SplashScreenView,

    pub all_users: Vec<User>,

    pub config: Config,

}

impl Greeter {
    fn new(config: &Config) -> Self {
        Greeter {
            view: View::default(),

            user_select_view: UserSelectView::default(),
            user_create_view: UserCreateView::default(),
            splash_screen_view: SplashScreenView::default(),

            all_users: User::get_all(),

            config: config.clone(),
        }
    }
}

#[derive(Debug)]
pub enum Message {
    UserSelectViewMessage(UserSelectViewMessage),
    UserCreateViewMessage(UserCreateViewMessage),
    SplashScreenViewMessage(SplashScreenViewMessage)
}

impl Application for Greeter {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Config;

    fn new(flags: Config) -> (Self, Command<Message>) {
        (
            Greeter::new(&flags),
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Sintonis Greeter")
    }


    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::UserSelectViewMessage(msg) => {
                if let UserSelectViewMessage::Action(action) = &msg {
                    match action {
                        Action::SwitchView(view) => self.view = *view,
                    }
                }
                
                return self
                    .user_select_view
                    .update(msg)
                    .map(Message::UserSelectViewMessage);
            },
            Message::UserCreateViewMessage(msg) => {
                if let UserCreateViewMessage::Action(action) = &msg {
                    match action {
                        Action::SwitchView(view) => self.view = *view,
                    }
                }
                
                return self
                    .user_create_view
                    .update(msg)
                    .map(Message::UserCreateViewMessage);
            },
            Message::SplashScreenViewMessage(msg) => {
                if let SplashScreenViewMessage::Action(action) = &msg {
                    match action {
                        Action::SwitchView(view) => self.view = *view,
                    }
                }

                return self
                    .splash_screen_view
                    .update(msg)
                    .map(Message::SplashScreenViewMessage);
            },
        }
    }

    fn view(&self) -> Element<Self::Message> {
        return match self.view {
            View::UserCreate => self.user_create_view.view().map(Message::UserCreateViewMessage),
            View::UserSelect => self.user_select_view.view(&self.all_users).map(Message::UserSelectViewMessage),
            View::Splash => self.splash_screen_view.view().map(Message::SplashScreenViewMessage)
        };
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        return match self.view {
            View::UserCreate => self.user_create_view.subscription().map(Message::UserCreateViewMessage),
            View::UserSelect => self.user_select_view.subscription().map(Message::UserSelectViewMessage),
            View::Splash => self.splash_screen_view.subscription().map(Message::SplashScreenViewMessage)

        }
    }
}

fn settings(config: Config) -> Settings<Config> {
    use iced::window::Settings as WindowSettings;
    Settings {
        window: WindowSettings {
            size: Size::new(800.0, 400.0),
            resizable: true,
            decorations: true,
            transparent: false,
            level: Default::default(),
            icon: None,
            platform_specific: Default::default(),
            min_size: Some(Size::new(800.0, 400.0)),
            max_size: None,
            position: Default::default(),
            visible: false,
            exit_on_close_request: true,
        },
        flags: config,
        fonts: vec![],
        default_font: Font::DEFAULT,
        default_text_size: Pixels::from(32.0),
        antialiasing: false,
        id: Some("sintonis-greeter".to_string()),
    }
}
