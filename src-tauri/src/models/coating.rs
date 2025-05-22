use diesel::prelude::Queryable;
use serde::Serialize;

#[derive(Queryable,Serialize)]
pub struct Coating {
    name:String
}