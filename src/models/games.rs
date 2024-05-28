use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::developers::Developer;
use super::genres::Genre;
use super::platforms::{ParentPlatform, Platform};
use super::publishers::Publisher;
use super::stores::Store;
use super::tags::Tag;

#[serde_with::apply(
    Option => #[serde_as(as = "NoneAsEmptyString")],
)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub id: u32,
    pub slug: String,
    pub name: String,
    pub name_original: Option<String>,
    pub description: Option<String>,
    pub metacritic: Option<u16>,
    pub metacritic_platforms: Option<Vec<MetacriticPlatform>>,
    pub released: Option<String>,
    pub tba: Option<bool>,
    pub updated: Option<String>,
    pub background_image: Option<String>,
    pub background_image_additional: Option<String>,
    pub website: Option<String>,
    pub rating: Option<f32>,
    pub rating_top: Option<u16>,
    pub ratings: Option<Vec<Rating>>,
    // pub reactions: Option<HashMap<String, u32>>,   /// !TODO
    //pub added: Option<i32>,
    // pub added_by_status: Option<AddedByStatus>,
    //pub playtime: Option<i32>, //IN HOURS
    pub screenshots_count: Option<i32>,
    pub movies_count: Option<i32>,
    pub creators_count: Option<i32>,
    pub achievements_count: Option<i32>,
    pub parent_achievements_count: Option<i32>,
    pub reddit_url: Option<String>, /* sub name or address */
    pub reddit_name: Option<String>,
    pub reddit_description: Option<String>,
    pub reddit_logo: Option<String>,
    pub reddit_count: Option<i32>,
    pub twitch_count: Option<i32>,
    pub youtube_count: Option<i32>,
    //pub reviews_text_count: Option<i32>,
    //pub ratings_count: Option<i32>,
    //pub suggestions_count: Option<i32>,
    pub alternative_names: Option<Vec<String>>,
    pub metacritic_url: Option<String>,
    pub parents_count: Option<i32>,
    pub additions_count: Option<i32>,
    pub game_series_count: Option<i32>,
    // pub  user_game: ???,  /// TODO!
    //pub reviews_count: Option<i32>,
    pub saturated_color: Option<String>,
    pub dominant_color: Option<String>,
    pub parent_platforms: Vec<ParentPlatform>,
    pub platforms: Option<Vec<Platform>>,
    pub stores: Option<Vec<Store>>,
    pub developers: Option<Vec<Developer>>,
    pub genres: Option<Vec<Genre>>,
    pub tags: Option<Vec<Tag>>,
    pub publishers: Option<Vec<Publisher>>,
    pub esrb_rating: Option<EsrbRating>,
    // pub clip: ???,
    pub description_raw: Option<String>,
}

#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Serialize, Deserialize, Debug)]
pub struct AddedByStatus {
    pub yet: Option<i32>,
    pub owned: Option<i32>,
    pub beaten: Option<i32>,
    pub toplay: Option<i32>,
    pub dropped: Option<i32>,
    pub playing: Option<i32>,
}

#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Rating {
    pub id: Option<u32>,
    pub title: Option<String>,
    pub count: Option<i32>,
    pub percent: Option<f32>,
}

#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Serialize, Deserialize, Debug)]
pub struct EsrbRating {
    pub id: Option<u32>,
    pub slug: Option<String>,
    pub name: Option<String>,
}

#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Serialize, Deserialize, Debug)]
pub struct MetacriticPlatform {
    pub metascore: Option<i32>,
    pub url: Option<String>,
}
