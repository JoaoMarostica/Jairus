use diesel::prelude::{AsChangeset, Insertable, Queryable, Selectable};

#[derive(Queryable,Selectable,Insertable,AsChangeset)]
#[diesel(table_name = crate::schema::tb_seed)]
pub struct Seed {
    scientific_name:String,
    popular_name:String
}

impl Seed {
    pub fn new(id:&str, name:&str) -> Self {
        Seed {
            scientific_name: id.to_string(),
            popular_name: name.to_string()
        }
    }

    pub fn id(&self) -> &String {
        &self.scientific_name
    }

    pub fn name(&self) -> &String {
        &self.popular_name
    }
}