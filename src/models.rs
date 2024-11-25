use serde::{Deserialize, Serialize};
use sqlx::FromRow;


/*For the Item database */
#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub quantity: i32,
    pub price: f64,
}

#[derive(Deserialize, Debug)]
pub struct NewItem {
    pub name: String,
    pub quantity: i32,
    pub price: f64
}

