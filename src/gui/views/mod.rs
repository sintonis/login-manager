pub mod user_select;
pub mod user_create;
pub mod splash_screen;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum View {
    UserCreate,
    UserSelect,
    Splash,
}

impl Default for View {
    fn default() -> Self {
        Self::UserSelect
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    SwitchView(View),
}