use config::Config;
use crate::utils::user::User;
use iced::{Element, Sandbox, Settings, Theme};
use iced::widget::{Button, Column, Container, Text};

mod config;
mod log;
mod utils;
mod constants;
mod gui;
mod assets;

fn main() -> anyhow::Result<()> {
    let conf = Config::from_any().unwrap();
    log::setup(&conf.log);
    
    // let users: Vec<User> = User::get_all();
    
    // Setup gui
    gui::run(conf);

    Ok(())
}