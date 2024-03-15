
pub struct LibrarySystem {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub main_branch: Library,
}

pub struct Library {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub street_address: String,
    pub city: String,
    pub state_or_province: String,
    pub postal_code: String,
    pub phone_number: String,
    pub point_balance: u64,
}

pub struct Quest {
    pub id: String,
    pub library_id: String,
    pub book_name: String,
    pub author: String,
    pub book_series: Option<String>,
    pub point_value: u64,
}

