pub struct Quest {
    pub id: String,
    pub library_id: String,
    pub book_name: String,
    pub author: String,
    pub book_series: Option<String>,
    pub point_value: u64,
}