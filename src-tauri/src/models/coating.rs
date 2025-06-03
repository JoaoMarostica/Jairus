use diesel::prelude::{AsChangeset, Insertable, Queryable, Selectable};

#[derive(Queryable,Selectable,Insertable,AsChangeset)]
#[diesel(table_name = crate::schema::tb_coating)]
pub struct Coating {
    coating_name:String
}

impl Coating {
    pub fn new(id:&str) -> Self {
        Coating { coating_name: id.to_string() }
    }

    pub fn id(&self) -> &String {
        &self.coating_name
    }
}