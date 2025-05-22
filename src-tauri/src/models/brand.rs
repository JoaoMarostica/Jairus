use diesel::prelude::Queryable;
use serde::Serialize;

#[derive(Queryable,Serialize)]
pub struct Brand {
    name:String,
    sack_weight:Vec<u32>
}