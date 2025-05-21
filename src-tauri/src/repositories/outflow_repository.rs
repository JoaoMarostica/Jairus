use crate::models::outflow::Outflow;

pub struct OutflowRepository;

impl OutflowRepository {
    fn create(object:&Outflow) {

    }

    fn read(id:&u32) -> Option<Outflow> {
        None
    }

    fn read_all(id:&(u32,u32)) -> Vec<Outflow> {
        vec![]
    }

    fn update(id:&u32, object:&Outflow) {

    }

    fn delete(id:&u32) {

    }
}