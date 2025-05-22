use serde::Serialize;

#[derive(Serialize)]
pub struct Balance {
    sack_amount: u32,
    total_weight: u32,
    total_pureness_score: f32
}