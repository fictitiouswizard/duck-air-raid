use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(any(test, feature = "ssr"))]
use {
    sqlx
};


#[cfg_attr(any(feature = "ssr", test), derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize)]
pub struct Library {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub street_address: String,
    pub city: String,
    pub state_or_province: String,
    pub postal_code: String,
    pub point_balance: u64,
}