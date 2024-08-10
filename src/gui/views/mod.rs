pub mod user_select;
pub mod user_create;
pub mod user_login;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum View {
    UserCreate,
    UserLogin,
    UserSelect,
}

impl Default for View {
    fn default() -> Self {
        Self::UserCreate
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    SwitchView(View),
}