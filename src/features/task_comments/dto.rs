use serde::Deserialize;



#[derive(Deserialize)]
pub struct ListCommentsDto {
    pub created_at_ascending: bool,
    pub offset: i32,
    pub limit: i32,
}


// ------- Implementations ------- //


impl ListCommentsDto {
    /// Validate the ListCommentsDto
    pub fn validate(&self) -> bool {
        // Can be only positive values for offset and limit
        if self.offset < 0 || self.limit <= 0 {
            return false;
        }

        // Per page it cant exceed a reasonable limit of 50
        if self.limit > 50 {
            return false;
        }

        true
    }
}
