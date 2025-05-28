use diesel::prelude::{AsChangeset, Insertable, Queryable, Selectable};

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
}

#[derive(Queryable,Selectable,Insertable,AsChangeset)]
#[diesel(table_name = crate::schema::tb_brand)]
pub struct VBrand {
    pub brand_name:String,
    pub sack_weight:i32
}