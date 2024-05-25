use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_with::apply(
    Option => #[serde_as(as = "NoneAsEmptyString")],
)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Creator {
    pub id: u32,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub image: Option<String>,
    pub image_background: Option<String>,
    pub description: Option<String>,
    pub games_count: Option<u32>,
    pub reviews_count: Option<u32>,
    pub rating: Option<f32>,
    pub rating_top: Option<u32>,
    pub updated: Option<String>,
}
