use diesel::prelude::{AsChangeset, Insertable, Queryable, Selectable};

#[derive(Queryable,Selectable,Insertable,AsChangeset)]
#[diesel(table_name = crate::schema::tb_batch)]
pub struct Batch {
    batch_number:i32,
    batch_year:i32,
    batch_month:i32,
    seed:String,
    coating:String,
    brand:String,
    sack_weight:i32,
    sack_amount: i32,
    total_weight: i32,
    pureness_score:f32,
    total_pureness_score: f32,
    origin:String
}

impl Batch {
    pub fn new(
        batch_number:i32,
        batch_year:i32,
        batch_month:i32,
        seed:String,
        coating:String,
        brand:String,
        sack_weight:i32,
        sack_amount:i32,
        total_weight:i32,
        pureness_score:f32,
        total_pureness_score:f32,
        origin:String) -> Self {
            Batch {
                batch_number:batch_number,
                batch_year:batch_year,
                batch_month:batch_month,
                seed:seed,
                coating:coating,
                brand:brand,
                sack_weight:sack_weight,
                sack_amount:sack_amount,
                total_weight:total_weight,
                pureness_score:pureness_score,
                total_pureness_score:total_pureness_score,
                origin:origin
            }
        }
}