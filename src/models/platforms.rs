use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_with::apply(
    Option => #[serde_as(as = "NoneAsEmptyString")],
)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Platforms {
    pub platform: Platform,
    pub released_at: Option<String>,
    pub requirements: Requirements,
}

#[serde_with::apply(
    Option => #[serde_as(as = "NoneAsEmptyString")],
)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Platform {
    pub platform: PlatformDetails,
    pub released_at: Option<String>,
    pub requirements: Requirements,
}

#[serde_with::apply(
    Option => #[serde_as(as = "NoneAsEmptyString")],
)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PlatformDetails {
    pub id: u32,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub image: Option<String>,
    pub year_end: Option<String>,
    pub year_start: Option<String>,
    pub games_count: Option<u32>,
    pub image_background: Option<String>,
    pub released_at: Option<String>,
    pub requirements: Option<Requirements>,
}

#[serde_with::apply(
    Option => #[serde_as(as = "NoneAsEmptyString")],
)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ParentPlatform {
    pub platform: PlatformShort,
}

#[serde_with::apply(
    Option => #[serde_as(as = "NoneAsEmptyString")],
)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PlatformShort {
    pub id: u32,
    pub name: Option<String>,
    pub slug: Option<String>,
}

#[serde_with::apply(
    Option => #[serde_as(as = "NoneAsEmptyString")],
)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Requirements {
    pub minimum: Option<String>,
    pub recommended: Option<String>,
}
