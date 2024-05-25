use crate::error::GetError;
use crate::Rawg;

use crate::models::genres::*;

pub struct GenresHandler<'instance> {
    instance: &'instance Rawg,
}

impl<'instance> GenresHandler<'instance> {
    pub(crate) fn new(instance: &'instance Rawg) -> Self {
        Self { instance }
    }

    pub async fn by_id(&self, id: i32) -> Result<Genre, GetError> {
        let route = format!("/genres/{id}");

        self.instance.get(route).await
    }
}
