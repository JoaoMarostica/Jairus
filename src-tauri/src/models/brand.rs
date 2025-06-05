use diesel::prelude::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Brand {
    brand_name:String,
    weights:Vec<i32>
}

impl Brand {
    pub fn new(id:&str, weights:Vec<i32>) -> Self {
        Brand { brand_name: id.to_string(), weights: weights}
    }
    pub fn id(&self) -> &String {
        &self.brand_name
    }

    pub fn weights(&mut self) -> &mut Vec<i32> {
        &mut self.weights
    }

    pub fn to_vbrand(&self) -> Vec<VBrand> {
        let mut vbrands:Vec<VBrand> = vec![];
        for weight in &self.weights {
            vbrands.push(VBrand {
                brand_name: self.brand_name.to_string(),
                sack_weight: weight.clone()
            })
        }
        vbrands
    }
}

#[derive(Queryable,Selectable,Insertable,AsChangeset)]
#[diesel(table_name = crate::schema::tb_brand)]
pub struct VBrand {
    pub brand_name:String,
    pub sack_weight:i32
}