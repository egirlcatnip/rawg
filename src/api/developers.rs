use crate::error::GetError;
use crate::Rawg;

use crate::models::developers::*;

pub struct DevelopersHandler<'instance> {
    instance: &'instance Rawg,
}

impl<'instance> DevelopersHandler<'instance> {
    pub(crate) fn new(instance: &'instance Rawg) -> Self {
        Self { instance }
    }

    pub async fn by_id(&self, id: i32) -> Result<Developer, GetError> {
        let route = format!("/developers/{id}");

        self.instance.get(route).await
    }
}
