use crate::error::GetError;
use crate::Rawg;

use crate::models::creators::*;

pub struct CratorsHandler<'instance> {
    instance: &'instance Rawg,
}

impl<'instance> CratorsHandler<'instance> {
    pub(crate) fn new(instance: &'instance Rawg) -> Self {
        Self { instance }
    }

    pub async fn by_id(&self, id: i32) -> Result<Creator, GetError> {
        let route = format!("/creators/{id}");

        self.instance.get(route).await
    }
}
