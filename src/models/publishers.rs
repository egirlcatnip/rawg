use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_with::apply(
    Option => #[serde_as(as = "NoneAsEmptyString")],
)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Publisher {
    id: u16,
    name: Option<String>,
    slug: Option<String>,
    games_count: Option<u32>,
    image_background: Option<String>,
    description: Option<String>,
}
