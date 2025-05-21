use diesel::prelude::Queryable;
use serde::Serialize;

#[derive(Queryable,Serialize)]
pub struct Brand {
    name:String,
    bag_weight:Vec<u32>
}