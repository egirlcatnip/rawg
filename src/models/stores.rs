use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_with::apply(
    Option => #[serde_as(as = "NoneAsEmptyString")],
)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
    id: u32,
    name: Option<String>,
    slug: Option<String>,
    domain: Option<String>,
    games_count: Option<u32>,
    image_background: Option<String>,
    description: Option<String>,
}
