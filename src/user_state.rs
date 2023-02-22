use eframe::egui;
use std::time::{Duration, Instant};


#[derive(Clone, PartialEq)]
pub enum UserStatus {
    LoggedIn,
    Away,
    LoggedOut,
}

impl UserStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserStatus::LoggedIn => "Logged In",
            UserStatus::Away => "Away",
            UserStatus::LoggedOut => "Logged Out",
        }
    }

    pub fn color(&self) -> egui::Color32 {
        match self {
            UserStatus::LoggedIn => egui::Color32::LIGHT_GREEN,
            UserStatus::Away => egui::Color32::LIGHT_YELLOW,
            UserStatus::LoggedOut => egui::Color32::LIGHT_RED,
        }
    }
}

#[derive(Clone)]
pub struct UserState{
    pub name: String,
    pub user_status: UserStatus,
    pub last_interaction_time: Instant,
}

impl Default for UserState{
    fn default() -> Self{
        Self {
            name: "".to_owned(),
            user_status: UserStatus::LoggedOut,
            last_interaction_time: Instant::now(),
        }
    }
}