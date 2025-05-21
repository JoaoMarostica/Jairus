use crate::models::process::Process;

pub struct ProcessRepository;

impl ProcessRepository {
    fn Create(object:&Process) {

    }

    fn Read(id:&str) -> Option<Process> {
        None
    }

    fn ReadAll() -> Vec<Process> {
        vec![]
    }

    fn Update(id:&str, object:&Process) {

    }

    fn Delete(id:&str) {

    }
}