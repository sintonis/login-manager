use config::Config;
use crate::utils::user::User;

mod config;
mod log;
mod utils;
mod constants;

fn main() -> anyhow::Result<()> {
    let conf = Config::from_any().unwrap();
    log::setup(&conf.log);
    
    // let users: Vec<User> = User::get_all();
    
    // Setup gui
    

    Ok(())
}