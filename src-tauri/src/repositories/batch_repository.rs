use crate::models::batch::Batch;

pub struct BatchRepository;

impl BatchRepository {
    fn create(object:&Batch) {

    }

    fn read(id:&(u32,u32)) -> Option<Batch> {
        None
    }

    fn read_all() -> Vec<Batch> {
        vec![]
    }

    fn update(id:&(u32,u32), object:&Batch) {

    }

    fn delete(id:&(u32,u32)) {

    }
}