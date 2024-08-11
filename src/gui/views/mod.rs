pub mod user;
pub mod splash_screen;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum View {
    User,
    Splash,
}

impl Default for View {
    fn default() -> Self {
        Self::User
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    SwitchView(View),
}