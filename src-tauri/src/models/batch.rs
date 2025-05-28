use diesel::prelude::{AsChangeset, Insertable, Queryable, Selectable};

#[derive(Queryable,Selectable,Insertable,AsChangeset)]
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
    pub origin: String
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
        origin: String) -> Self {
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
                origin
            }
        }
}