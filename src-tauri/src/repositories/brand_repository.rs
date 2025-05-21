use crate::models::brand::Brand;

pub struct BrandRepository;

impl BrandRepository {
    fn create(object:&Brand) {

    }

    fn read(id:&str) -> Option<Brand> {
        None
    }

    fn read_all() -> Vec<Brand> {
        vec![]
    }

    fn update(id:&str, object:&Brand) {

    }

    fn delete(id:&str) {

    }
}