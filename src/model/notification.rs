use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Notification {
    pub product_tittle: String,
    pub product_type: String,
    pub product_url: String,
    pub subscriber_name: String,
    pub status: String
}