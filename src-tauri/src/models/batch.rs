use diesel::prelude::Queryable;
use serde::Serialize;

use crate::models::{balance::Balance, seed::Seed, process::Process, brand::Brand};

#[derive(Queryable,Serialize)]
pub struct Batch {
    batch_number:u32,
    year:u32,
    month:u32,
    seed:Seed,
    processing:Process,
    brand:Brand,
    bag_weight:u32,
    pureness_score:f32,
    initial_balance:Balance,
    origin:String
}