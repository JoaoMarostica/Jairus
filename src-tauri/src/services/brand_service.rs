use crate::{
    models::brand::{
        Brand,
        VBrand
    },
    repositories::brand_repository::BrandRepository
};

pub struct BrandService {
    repo:BrandRepository
}

impl BrandService {
    pub fn new() -> BrandService {
        BrandService { repo: BrandRepository::new() }
    }

    pub fn create(&mut self, new:&Brand) -> Vec<String> {
        let mut response:Vec<String> = vec![];
        for vb in new.to_vbrand() {
            match self.repo.insert(&vb) {
                Ok(result) => response.push(
                    format!("Added {}-{} to database\n",result.brand_name, result.sack_weight)),
                Err(e) => response.push(e.to_string())
            }
        }
        response
    }

    pub fn create_value(&mut self, id:&str, value:i32) -> String {
        match self.repo.insert(&VBrand {
            brand_name: id.to_string(),
            sack_weight: value }) {
            Ok(vb) => format!("Added {} to {}", vb.brand_name, vb.sack_weight),
            Err(e) => e.to_string()
        }
    }

    pub fn read_id(&mut self, id:&str) -> Result<Brand, String> {
        match self.repo.select_id(id) {
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

    pub fn read_all(&mut self) -> Result<Vec<Brand>, String> {
        match self.repo.select_all() {
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

    pub fn update(&mut self, id:&str, new_name:&str) -> Result<Brand, String> {
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

    pub fn delete(&mut self, id:&str) -> Result<Brand, String> {
        match self.repo.drop(id) {
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

    pub fn delete_value(&mut self, id:&str, value:i32) -> Result<i32, String> {
        match self.repo.drop_value(id, value) {
            Ok(Some(result)) => {
                Ok(result.sack_weight)
            },
            Ok(None) => Err(format!("No brand with id {} found",id)),
            Err(e) => Err(e.to_string())
        }
    }
}