use crate::error::GetError;
use crate::Rawg;

use crate::models::tags::*;

pub struct TagsHandler<'instance> {
    instance: &'instance Rawg,
}

impl<'instance> TagsHandler<'instance> {
    pub(crate) fn new(instance: &'instance Rawg) -> Self {
        Self { instance }
    }

    pub async fn by_id(&self, id: i32) -> Result<Tag, GetError> {
        let route = format!("/tags/{id}");

        self.instance.get(route).await
    }
}
