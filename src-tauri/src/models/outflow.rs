use diesel::prelude::Queryable;
use serde::Serialize;

use crate::models::balance::Balance;

#[derive(Queryable,Serialize)]
pub struct Outflow {
    id:u32,
    outflow_balance:Balance,
    usage:String
}