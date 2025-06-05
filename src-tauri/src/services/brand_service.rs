use crate::{
    models::brand::Brand,
    repositories::brand_repository::BrandRepository
};

pub struct BrandService {
    repo:BrandRepository
}

impl BrandService {
    pub fn new() -> BrandService {
        BrandService { repo: BrandRepository::new() }
    }

    pub fn add(&mut self, new:&Brand) -> Vec<String> {
        let mut response:Vec<String> = vec![];
        for vb in new.to_vbrand() {
            match self.repo.create(&vb) {
                Ok(result) => response.push(
                    format!("Added {}-{} to database\n",result.brand_name, result.sack_weight)),
                Err(e) => response.push(e.to_string())
            }
        }
        response
    }

    pub fn get(&mut self, id:&str) -> Result<Brand, String> {
        match self.repo.read(id) {
            Ok(Some(result)) => {
                let mut weights:Vec<i32> = vec![];
                for vb in result {
                    weights.push(vb.sack_weight);
                }
                Ok(Brand::new(id, weights))
            },
            Ok(None) => Err(format!("No brand with id {} found",id)),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn list(&mut self) -> Result<Vec<Brand>, String> {
        match self.repo.read_all() {
            Ok(Some(result)) => {
                let mut brands:Vec<Brand> = vec![];
                for vb in result {
                    match brands.iter_mut()
                    .find(|x| x.id() == &vb.brand_name) {
                        Some(b) => b.weights().push(vb.sack_weight),
                        None => brands.push(
                            Brand::new(
                                &vb.brand_name,
                                vec![vb.sack_weight]
                            )),
                    }
                }
                Ok(brands)
            },
            Ok(None) => Err("No brands in the database".to_string()),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn save(&mut self, id:&str, new_name:&str) -> Result<Brand, String> {
        match self.repo.update(id, new_name) {
            Ok(Some(result)) => {
                let mut weights:Vec<i32> = vec![];
                for vb in result {
                    weights.push(vb.sack_weight);
                }
                Ok(Brand::new(id, weights))
            },
            Ok(None) => Err(format!("No brand with id {} found",id)),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn remove(&mut self, id:&str) -> Result<Brand, String> {
        match self.repo.delete(id) {
            Ok(Some(result)) => {
                let mut weights:Vec<i32> = vec![];
                for vb in result {
                    weights.push(vb.sack_weight);
                }
                Ok(Brand::new(id, weights))
            },
            Ok(None) => Err(format!("No brand with id {} found",id)),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn remove_part(&mut self, id:&str, value:i32) -> Result<i32, String> {
        match self.repo.delete_subvalue(id, value) {
            Ok(Some(result)) => {
                Ok(result.sack_weight)
            },
            Ok(None) => Err(format!("No brand with id {} found",id)),
            Err(e) => Err(e.to_string())
        }
    }
}