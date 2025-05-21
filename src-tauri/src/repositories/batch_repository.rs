use crate::models::batch::Batch;

pub struct BatchRepository;

impl BatchRepository {
    fn Create(object:&Batch) {

    }

    fn Read(id:&(u32,u32)) -> Option<Batch> {
        None
    }

    fn ReadAll() -> Vec<Batch> {
        vec![]
    }

    fn Update(id:&(u32,u32), object:&Batch) {

    }

    fn Delete(id:&(u32,u32)) {

    }
}