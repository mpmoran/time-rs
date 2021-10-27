mod application;
mod configuration_repository;
mod csv_repository;
mod settings_view;
mod stopwatch;
mod task_record;
mod time_rs;

use iced::{Application, Result, Settings};

pub fn main() -> Result {
    application::Application::run(Settings::default())
}
