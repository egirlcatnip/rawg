use crate::error::GetError;
use crate::Rawg;

use crate::models::publishers::*;

pub struct PublishersHandler<'instance> {
    instance: &'instance Rawg,
}

impl<'instance> PublishersHandler<'instance> {
    pub(crate) fn new(instance: &'instance Rawg) -> Self {
        Self { instance }
    }

    pub async fn by_id(&self, id: i32) -> Result<Publisher, GetError> {
        let route = format!("/publishers/{id}");

        self.instance.get(route).await
    }
}
