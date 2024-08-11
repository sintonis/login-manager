mod panels;
mod views;

use alloc::borrow::Cow;
use iced::{Application, Command, Element, Font, Pixels, Settings, Size, Subscription, Theme};
use crate::config::Config;
use crate::gui::views::{Action, View};
use crate::gui::views::splash_screen::{SplashScreenView, SplashScreenViewMessage};
use crate::gui::views::user::{UserView, UserViewMessage};

pub fn run(config: Config) -> Result<(), ()> {
    Greeter::run(settings(config)).map_err(|_| ())
}

#[derive(Debug)]
pub struct Greeter {
    pub view: View,

    pub user_view: UserView,
    pub splash_screen_view: SplashScreenView,

    pub config: Config,

}

impl Greeter {
    fn new(config: &Config) -> Self {
        Greeter {
            view: View::default(),

            user_view: UserView::new(),
            splash_screen_view: SplashScreenView::default(),

            config: config.clone(),
        }
    }
}

#[derive(Debug)]
pub enum Message {
    UserViewMessage(UserViewMessage),
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
            Message::UserViewMessage(msg) => {
                if let UserViewMessage::Action(action) = &msg {
                    match action {
                        Action::SwitchView(view) => self.view = *view,
                    }
                }
                
                return self
                    .user_view
                    .update(msg)
                    .map(Message::UserViewMessage);
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
            View::User => self.user_view.view().map(Message::UserViewMessage),
            View::Splash => self.splash_screen_view.view().map(Message::SplashScreenViewMessage)
        };
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        return match self.view {
            View::User => self.user_view.subscription().map(Message::UserViewMessage),
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
            transparent: true,
            level: Default::default(),
            icon: None,
            platform_specific: Default::default(),
            min_size: Some(Size::new(800.0, 480.0)),
            max_size: None,
            position: Default::default(),
            visible: true,
            exit_on_close_request: true,
        },
        flags: config,
        fonts: vec![
            Cow::from(include_bytes!("../../assets/fonts/Roboto-Black.ttf")),
            Cow::from(include_bytes!("../../assets/fonts/Roboto-Bold.ttf")),
            Cow::from(include_bytes!("../../assets/fonts/Roboto-Medium.ttf")),
            Cow::from(include_bytes!("../../assets/fonts/Roboto-Regular.ttf")),
            Cow::from(include_bytes!("../../assets/fonts/Roboto-Thin.ttf")),
            Cow::from(include_bytes!("../../assets/fonts/Roboto-ThinItalic.ttf")),
            Cow::from(include_bytes!("../../assets/fonts/Roboto-Italic.ttf")),
            Cow::from(include_bytes!("../../assets/fonts/Roboto-Light.ttf")),
            Cow::from(include_bytes!("../../assets/fonts/Roboto-LightItalic.ttf")),
            Cow::from(include_bytes!("../../assets/fonts/Roboto-BlackItalic.ttf")),
        ],
        default_font: Font::DEFAULT,
        default_text_size: Pixels::from(16.0),
        antialiasing: false,
        id: Some("sintonis-greeter".to_string()),
    }
}
