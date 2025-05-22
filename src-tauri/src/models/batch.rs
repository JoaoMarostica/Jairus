use diesel::prelude::Queryable;
use serde::Serialize;

use crate::models::{balance::Balance, seed::Seed, coating::Coating, brand::Brand};

#[derive(Queryable,Serialize)]
pub struct Batch {
    batch_number:u32,
    year:u32,
    month:u32,
    seed:Seed,
    coating:Coating,
    brand:Brand,
    sack_weight:u32,
    pureness_score:f32,
    initial_balance:Balance,
    origin:String
}