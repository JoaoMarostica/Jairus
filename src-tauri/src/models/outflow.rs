use diesel::prelude::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use super::balance::Balance;

#[derive(Queryable, Selectable, AsChangeset, Serialize)]
#[diesel(table_name = crate::schema::tb_outflow)]
pub struct Outflow {
    outflow_id:i32,
    batch_number:i32,
    batch_year:i32,
    sack_amount: i32,
    total_weight: i32,
    total_pureness_score: f32,
    usage:String
}

impl Outflow {
    pub fn new(
        id:i32,
        batch_id:(i32,i32),
        sack_amount:i32,
        total_weight:i32,
        total_pureness_score:f32,
        usage:&str) -> Self {
            let (bn, by) = batch_id;
            Outflow {
                outflow_id: id,
                batch_number:bn,
                batch_year:by,
                sack_amount: sack_amount,
                total_weight: total_weight,
                total_pureness_score: total_pureness_score,
                usage: usage.to_string()
            }
    }

    pub fn id(&self) -> &i32 {
        &self.outflow_id
    }

    pub fn get_balance(&self) -> Balance {
        Balance::new(self.sack_amount, self.total_weight, self.total_pureness_score)
    }
}

#[derive(Insertable,AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::tb_outflow)]
pub struct NewOutflow {
    batch_number:i32,
    batch_year:i32,
    sack_amount: i32,
    total_weight: i32,
    total_pureness_score: f32,
    usage:String
}

impl NewOutflow {
    pub fn new(
        batch_id:(i32,i32),
        sack_amount:i32,
        total_weight:i32,
        total_pureness_score:f32,
        usage:&str) -> Self {
            let (bn, by) = batch_id;
            NewOutflow {
                batch_number:bn,
                batch_year:by,
                sack_amount: sack_amount,
                total_weight: total_weight,
                total_pureness_score: total_pureness_score,
                usage: usage.to_string() }
    }
}