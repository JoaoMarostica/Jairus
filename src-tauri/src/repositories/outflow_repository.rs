use crate::models::outflow::Outflow;

pub struct OutflowRepository;

impl OutflowRepository {
    fn Create(object:&Outflow) {

    }

    fn Read(id:&u32) -> Option<Outflow> {
        None
    }

    fn ReadAll(id:&(u32,u32)) -> Vec<Outflow> {
        vec![]
    }

    fn Update(id:&u32, object:&Outflow) {

    }

    fn Delete(id:&u32) {

    }
}