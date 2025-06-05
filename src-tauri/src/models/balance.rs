use serde::Serialize;

#[derive(Serialize)]
pub struct Balance {
    sack_amount:i32,
    total_weight:i32,
    total_pureness_score:f32
}

impl Balance {
    pub fn new(sack_amount:i32, total_weight:i32, total_pureness_score:f32) -> Balance {
        Balance {
            sack_amount: sack_amount,
            total_weight: total_weight,
            total_pureness_score: total_pureness_score
        }
    }
    
    pub fn sum(&mut self, other:Balance) {
        self.sack_amount += other.sack_amount;
        self.total_weight += other.total_weight;
        self.total_pureness_score += other.total_pureness_score;
    }
}