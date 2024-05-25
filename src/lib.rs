mod auth;
mod client;
mod error;
mod list;

pub mod api;
pub mod models;

use client::Rawg;


pub fn instance() -> Rawg {
    Rawg::default()
}

impl Rawg {
    pub fn crators(&self) -> api::creators::CratorsHandler {
        api::creators::CratorsHandler::new(self)
    }
    pub fn developers(&self) -> api::developers::DevelopersHandler {
        api::developers::DevelopersHandler::new(self)
    }
    pub fn games(&self) -> api::games::GamesHandler {
        api::games::GamesHandler::new(self)
    }
    pub fn genres(&self) -> api::genres::GenresHandler {
        api::genres::GenresHandler::new(self)
    }
    pub fn platforms(&self) -> api::platforms::PlatformsHandler {
        api::platforms::PlatformsHandler::new(self)
    }
    pub fn publishers(&self) -> api::publishers::PublishersHandler {
        api::publishers::PublishersHandler::new(self)
    }
    pub fn stores(&self) -> api::stores::StoresHandler {
        api::stores::StoresHandler::new(self)
    }
    pub fn tags(&self) -> api::tags::TagsHandler {
        api::tags::TagsHandler::new(self)
    }
}
