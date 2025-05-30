use diesel::{prelude::*,SqliteConnection};
use diesel::sql_types::Text;
use dotenvy::dotenv;

use core::panic;
use std::env;

use crate::{schema::tb_batch::dsl::*,models::batch::*};

pub struct BatchRepository {
    connection:SqliteConnection
}

define_sql_function! {
    fn lower(x: Text) -> Text;
}

impl BatchRepository {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        Self { connection: connection }
    }

    pub fn create(&mut self, object:&Batch) -> Result<Batch, diesel::result::Error> {
        diesel::insert_into(tb_batch)
        .values(object)
        .returning(Batch::as_returning())
        .get_result(&mut self.connection)
    }

    pub fn read(&mut self, id:&(i32,i32)) -> Result<Option<Batch>, diesel::result::Error> {
        let (bn, by) = id;
        tb_batch.filter(
            batch_number.eq(bn)
            .and(batch_year.eq(by)))
        .select(Batch::as_select())
        .get_result(&mut self.connection)
        .optional()
    }

    pub fn read_all(&mut self) -> Result<Vec<Batch>, diesel::result::Error> {
        tb_batch .select(Batch::as_select())
        .get_results(&mut self.connection)
    }

    pub fn update(&mut self, id:&(i32, i32), object:&Batch) -> Result<Option<Batch>,diesel::result::Error> {
        let (bn, by) = id;
        diesel::update(tb_batch.filter(
            batch_number.eq(bn)
            .and(batch_year.eq(by))))
        .set(object)
        .returning(Batch::as_returning())
        .get_result(&mut self.connection)
        .optional()
    }

    pub fn delete(&mut self, id:&(i32,i32)) -> Result<Option<Batch>, diesel::result::Error>{
        let (bn, by) = id;
        diesel::delete(tb_batch.filter(
            batch_number.eq(bn)
            .and(batch_year.eq(by))))
        .get_result(&mut self.connection)
        .optional()
    }

    pub fn filter_batches(
        &mut self,
        query: &BatchQuery,
    ) -> Result<Vec<Batch>, diesel::result::Error> {
        let mut q = tb_batch.into_boxed(); // query dinâmica

        // Filtrar por ano
        if let Some(y) = query.year {
            q = q.filter(batch_year.eq(y));
        }

        // Filtro de busca
        if let Some(ref search_str) = query.search {
            let like_pattern = format!("%{}%", search_str.to_lowercase());

            if let Some(ref col) = query.column {
                match col.as_str() {
                    "batch_number" => {
                        // Como batch_number é inteiro, não faz sentido LIKE, só filtro exato
                        if let Ok(num) = search_str.parse::<i32>() {
                            q = q.filter(batch_number.eq(num));
                        }
                    }
                    "coating" => {
                        q = q.filter(lower(coating).like(like_pattern));
                    }
                    "seed" => {
                        q = q.filter(lower(seed).like(like_pattern));
                    }
                    "brand" => {
                        q = q.filter(lower(brand).like(like_pattern));
                    }
                    // Adicione mais colunas textuais permitidas se quiser
                    _ => {}
                }
            } else {
                // Busca geral em batch_number (exato) + coating (LIKE)
                // batch_number exato:
                let maybe_num = search_str.parse::<i32>().ok();

                if let Some(num) = maybe_num {
                    let like_pattern_owned = like_pattern.clone();
                    q = q.filter(batch_number.eq(num)
                        .or(lower(coating).like(like_pattern_owned))
                    );
                } else {
                    q = q.filter(lower(coating).like(like_pattern));
                }
            }
        }

        // Ordenação
        if let Some(ref sort_col) = query.sort_by {
            let order_dir = query.sort_order.as_deref().unwrap_or("asc");

            q = match (sort_col.as_str(), order_dir) {
                ("batch_number", "asc") => q.order(batch_number.asc()),
                ("batch_number", "desc") => q.order(batch_number.desc()),

                ("batch_year", "asc") => q.order(batch_year.asc()),
                ("batch_year", "desc") => q.order(batch_year.desc()),

                ("batch_month", "asc") => q.order(batch_month.asc()),
                ("batch_month", "desc") => q.order(batch_month.desc()),

                ("total_weight", "asc") => q.order(total_weight.asc()),
                ("total_weight", "desc") => q.order(total_weight.desc()),

                ("seed", "asc") => q.order(seed.asc()),
                ("seed", "desc") => q.order(seed.desc()),

                ("coating", "asc") => q.order(coating.asc()),
                ("coating", "desc") => q.order(coating.desc()),

                _ => q.order(batch_year.desc()), // default ordena por ano decrescente
            };
        } else {
            q = q.order(batch_year.desc()); // ordenação padrão
        }

        // Paginação
        let offset_val = query.page * query.page_size;

        q = q.limit(query.page_size).offset(offset_val);

        // Executa query e retorna
        q.load::<Batch>(&mut self.connection)
    }
}
