use iced::{Command, Element, Subscription};
use iced::widget::{container, text};
use crate::gui::views::Action;

#[derive(Debug, Default)]
pub struct SplashScreenView {}

#[derive(Debug, Clone)]
pub enum SplashScreenViewMessage {
    Action(Action)
}


impl SplashScreenView {
    pub fn update(&mut self, message: SplashScreenViewMessage) -> Command<SplashScreenViewMessage> {
        Command::none()
    }

    pub fn view(&self) -> Element<SplashScreenViewMessage> {
        container(text("Splash")).padding(20).into()
    }

    pub fn subscription(&self) -> Subscription<SplashScreenViewMessage> {
        Subscription::none()
    }
}





