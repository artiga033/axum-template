#[derive(Clone, Default)]
pub struct AppState {}

impl AppState {
    pub fn from_rt_config(_config: crate::config::Runtime) -> Self {
        AppState {}
    }
}
