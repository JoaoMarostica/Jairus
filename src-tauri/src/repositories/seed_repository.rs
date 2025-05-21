use crate::models::seed::Seed;

pub struct SeedRepository;

impl SeedRepository {
    fn Create(object:&Seed) {

    }

    fn Read(id:&str) -> Option<Seed> {
        None
    }

    fn ReadAll() -> Vec<Seed> {
        vec![]
    }

    fn Update(id:&str, object:&Seed) {

    }

    fn Delete(id:&str) {

    }
}