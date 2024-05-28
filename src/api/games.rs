use std::vec;

use crate::error::GetError;
use crate::list::List;
use crate::list::SearchHandler;
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

    pub fn search(&self, page_size: i32) -> SearchHandler<'instance> {
        SearchHandler {
            instance: self.instance,
            page_size: page_size,
        }
    }
}

impl<'instance> SearchHandler<'instance> {
    pub async fn query(&self, search_term: &str) -> Result<List<Game>, GetError> {
        let route = format!("games");
        let page_size = self.page_size.to_string();

        let query = Some(vec![
            ("search".to_string(), search_term.to_string()),
            ("page_size".to_string(), page_size),
        ]);

        self.instance
            .get_with_query::<List<Game>>(route, query)
            .await
    }
}
