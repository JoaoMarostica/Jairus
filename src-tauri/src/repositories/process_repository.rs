use crate::models::process::Process;

pub struct ProcessRepository;

impl ProcessRepository {
    fn create(object:&Process) {

    }

    fn read(id:&str) -> Option<Process> {
        None
    }

    fn read_all() -> Vec<Process> {
        vec![]
    }

    fn update(id:&str, object:&Process) {

    }

    fn delete(id:&str) {

    }
}