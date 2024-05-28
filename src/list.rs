use serde::{Deserialize, Serialize};

use crate::Rawg;

pub struct SearchHandler<'instance> {
    pub(crate) instance: &'instance Rawg,
    pub(crate) page_size: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct List<T> {
    pub count: i32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
}
