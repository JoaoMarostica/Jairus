use diesel::prelude::{AsChangeset, Insertable, Queryable, Selectable};

#[derive(Queryable,Selectable,AsChangeset)]
#[diesel(table_name = crate::schema::tb_outflow)]
pub struct Outflow {
    outflow_id:i32,
    sack_amount: i32,
    total_weight: i32,
    total_pureness_score: f32,
    usage:String
}

impl Outflow {
    pub fn new(id:i32, sack_amount:i32,
        total_weight:i32,
        total_pureness_score:f32,
        usage:&str) -> Self {
            Outflow {
                outflow_id: id,
                sack_amount: sack_amount,
                total_weight: total_weight,
                total_pureness_score: total_pureness_score,
                usage: usage.to_string() }
    }

    pub fn id(&self) -> &i32 {
        &self.outflow_id
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::tb_outflow)]
pub struct NewOutflow {
    sack_amount: i32,
    total_weight: i32,
    total_pureness_score: f32,
    usage:String
}

impl NewOutflow {
    pub fn new(sack_amount:i32,
        total_weight:i32,
        total_pureness_score:f32,
        usage:&str) -> Self {
            NewOutflow {
                sack_amount: sack_amount,
                total_weight: total_weight,
                total_pureness_score: total_pureness_score,
                usage: usage.to_string() }
    }
}