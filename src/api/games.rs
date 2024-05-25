use crate::error::GetError;
use crate::Rawg;

use crate::models::games::*;

pub struct GamesHandler<'instance> {
    instance: &'instance Rawg,
}

impl<'instance> GamesHandler<'instance> {
    pub(crate) fn new(instance: &'instance Rawg) -> Self {
        Self { instance }
    }

    pub async fn from_id(&self, id: i32) -> Result<Game, GetError> {
        let route = format!("/games/{id}");

        self.instance.get(route).await
    }

    pub async fn from_slug(&self, slug: String) -> Result<Game, GetError> {
        let route = format!("games/{slug}");

        self.instance.get(route).await
    }

    pub async fn id_from_slug(&self, slug: String) -> Result<u32, GetError> {
        let route = format!("games/{slug}");

        let game: Game = self.instance.get(route).await?;

        let id = game.id;

        Ok(id)
    }
}
