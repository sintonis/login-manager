extern crate alloc;

use config::Config;
use crate::utils::user::User;
use iced::{Element, Sandbox, Settings, Theme};

mod config;
mod log;
mod utils;
mod constants;
mod gui;

fn main() -> anyhow::Result<()> {
    let conf = Config::from_any().unwrap();
    log::setup(&conf.log);
    
    // let users: Vec<User> = User::get_all();
    
    // Setup gui
    gui::run(conf);

    Ok(())
}