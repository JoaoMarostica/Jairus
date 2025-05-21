use crate::models::seed::Seed;

pub struct SeedRepository;

impl SeedRepository {
    fn create(object:&Seed) {

    }

    fn read(id:&str) -> Option<Seed> {
        None
    }

    fn read_all() -> Vec<Seed> {
        vec![]
    }

    fn update(id:&str, object:&Seed) {

    }

    fn delete(id:&str) {

    }
}