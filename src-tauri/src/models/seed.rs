use diesel::prelude::{AsChangeset, Insertable, Queryable, Selectable};

#[derive(Queryable,Selectable,Insertable,AsChangeset)]
#[diesel(table_name = crate::schema::tb_seed)]
pub struct Seed {
    popular_name:String,
    scientific_name:String
}

impl Seed {
    pub fn new(id:&str, name:&str) -> Self {
        Seed {
            popular_name: name.to_string(),
            scientific_name: id.to_string()
        }
    }

    pub fn id(&self) -> &String {
        &self.popular_name
    }

    pub fn scientific_name(&self) -> &String {
        &self.scientific_name
    }
}