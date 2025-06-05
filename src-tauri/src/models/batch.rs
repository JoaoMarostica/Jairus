use diesel::prelude::{
    AsChangeset,
    Insertable,
    Queryable,
    Selectable
};
use serde::{
    Deserialize,
    Serialize
};
use super::balance::Balance;

#[derive(Queryable, Selectable, Insertable, AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::tb_batch)]
pub struct Batch {
    pub batch_number: i32,
    pub batch_year: i32,
    pub batch_month: i32,
    pub seed: String,
    pub coating: String,
    pub brand: String,
    pub sack_weight: i32,
    pub sack_amount: i32,
    pub total_weight: i32,
    pub pureness_score: f32,
    pub total_pureness_score: f32,
    pub batch_status: i32,
    pub deleted_at: Option<String>,
    pub origin: Option<String>
}

impl Batch {
    pub fn new(
        batch_number: i32,
        batch_year: i32,
        batch_month: i32,
        seed: String,
        coating: String,
        brand: String,
        sack_weight: i32,
        sack_amount: i32,
        total_weight: i32,
        pureness_score: f32,
        total_pureness_score: f32,
        batch_status: i32) -> Self {
            Batch {
                batch_number,
                batch_year,
                batch_month,
                seed,
                coating,
                brand,
                sack_weight,
                sack_amount,
                total_weight,
                pureness_score,
                total_pureness_score,
                batch_status,
                deleted_at:None,
                origin:None
            }
    }
    
    pub fn get_expiration_date(&self) -> String {
        format!("{}/{}", self.batch_month+1, self.batch_year+1)
    }
    
    pub fn get_initial_balance(&self) -> Balance {
        Balance::new(self.sack_amount, self.total_weight, self.total_pureness_score)
    }
}