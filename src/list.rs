/* use serde::{Deserialize, Serialize};
use serde_with::serde_as;

// use crate::error::ListError;

pub trait Listable<T> {
    async fn list(&self) -> List<T>;
}

#[serde_with::apply(
    Option => #[serde_as(as = "NoneAsEmptyString")],
)]
#[derive(Serialize, Deserialize, Debug)]
pub struct List<T> {
    pub page: String,
    pub count: Option<i32>,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Option<T>>,
}

impl<T> List<T> {
    pub fn next(self) -> Result<List<T>, ListError> {
        fn next_url<T>(list: List<T>) -> Result<String, ListError> {
            let next_page = list.next;

            match next_page {
                Some(Page) => Ok(Page),
                None => Err(ListError::NoPreviousPage),
            }
        }

        let route = next_url(self)?;

        todo!()
    }

    pub fn previous_url(self) -> Result<String, ListError> {
        let previous_page = self.previous;

        match previous_page {
            Some(Page) => Ok(Page),
            None => Err(ListError::NoPreviousPage),
        }
    }
    pub fn page(self, number: i32) -> Result<String, ListError> {
        todo!()
    }
} */
