use crate::error::GetError;
use crate::Rawg;

use crate::models::stores::*;

pub struct StoresHandler<'instance> {
    instance: &'instance Rawg,
}

impl<'instance> StoresHandler<'instance> {
    pub(crate) fn new(instance: &'instance Rawg) -> Self {
        Self { instance }
    }

    pub async fn by_id(&self, id: i32) -> Result<Store, GetError> {
        let route = format!("/stores/{id}");

        self.instance.get(route).await
    }
}
