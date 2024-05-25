use crate::error::GetError;
use crate::Rawg;

use crate::models::platforms::*;

pub struct PlatformsHandler<'instance> {
    instance: &'instance Rawg,
}

impl<'instance> PlatformsHandler<'instance> {
    pub(crate) fn new(instance: &'instance Rawg) -> Self {
        Self { instance }
    }

    pub async fn by_id(&self, id: i32) -> Result<Platform, GetError> {
        let route = format!("platforms/{id}");

        self.instance.get(route).await
    }
}
