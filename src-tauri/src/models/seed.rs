use diesel::prelude::Queryable;
use serde::Serialize;

#[derive(Queryable,Serialize)]
pub struct Seed {
    scientific_name:String,
    popular_name:String
}