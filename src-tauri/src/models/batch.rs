use diesel::prelude::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable,Selectable,Insertable,AsChangeset,Deserialize,Serialize)]
#[diesel(table_name = crate::schema::tb_batch)]
pub struct Batch {
    pub batch_number:i32,
    pub batch_year:i32,
    pub batch_month:i32,
    pub seed:String,
    pub coating:String,
    pub brand:String,
    pub sack_weight:i32,
    pub sack_amount:i32,
    pub total_weight:i32,
    pub pureness_score:f32,
    pub total_pureness_score:f32,
    pub batch_status:String,
    pub deleted_at:Option<String>,
    pub origin:Option<String>
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
        batch_status:String) -> Self {
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
}


#[derive(Deserialize)]
pub struct BatchQuery {
    pub search: Option<String>,
    pub column: Option<String>,
    pub year: Option<i32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: i64,
    pub page_size: i64,
}

impl BatchQuery {
    pub fn new(
        page: i64,
        page_size: i64) -> Self {
            BatchQuery {
                search:None,
                column:None,
                year:None,
                sort_by:None,
                sort_order:None,
                page,
                page_size
            }
        }
}