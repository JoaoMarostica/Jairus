use crate::models::coating::Coating;

pub struct CoatingRepository;

impl CoatingRepository {
    fn create(object:&Coating) {

    }

    fn read(id:&str) -> Option<Coating> {
        None
    }

    fn read_all() -> Vec<Coating> {
        vec![]
    }

    fn update(id:&str, object:&Coating) {

    }

    fn delete(id:&str) {

    }
}