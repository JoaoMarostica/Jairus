use crate::models::brand::Brand;

pub struct BrandRepository;

impl BrandRepository {
    fn Create(object:&Brand) {

    }

    fn Read(id:&str) -> Option<Brand> {
        None
    }

    fn ReadAll() -> Vec<Brand> {
        vec![]
    }

    fn Update(id:&str, object:&Brand) {

    }

    fn Delete(id:&str) {

    }
}