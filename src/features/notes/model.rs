use serde::{Deserialize, Serialize};
use pg_row_derive::RowFrom;



#[derive(Serialize, Deserialize, RowFrom)]
pub struct Notes {
    pub id: Option<i32>,    // If user wants to create we can use same struct
    pub title: String,
    pub content: String,
}
