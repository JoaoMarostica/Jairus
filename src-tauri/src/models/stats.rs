use serde::Serialize;

#[derive(Serialize)]
pub struct BatchStatistics {
    pub total_batches: usize,
    pub total_weight: i32,
    pub total_by_year: Vec<YearCount>,
    pub total_by_seed: Vec<SeedCount>,
}

#[derive(Serialize)]
pub struct YearCount {
    pub year: i32,
    pub count: usize,
    pub total_weight: i32,
}

#[derive(Serialize)]
pub struct SeedCount {
    pub seed: String,
    pub count: usize,
    pub total_weight: i32,
}
