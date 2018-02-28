#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{Tarea, NuevaTarea};

pub fn conectar() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("favor de agregar un DATABASE_URL");
    PgConnection::establish(&database_url)
        .expect(&format!("Error al conectar a {}", database_url))
}

pub fn crear_tarea<'a>(con: &PgConnection, tarea: &'a str) -> Tarea {
    use schema::tareas;

    let nueva_tarea = NuevaTarea {
        tarea: tarea,
    };

    diesel::insert_into(tareas::table)
        .values(&nueva_tarea)
        .get_result(con)
        .expect("Error al agregar tarea")

}

pub mod schema {
    table! {
        tareas (id) {
            id -> Int4,
            tarea -> Text,
            realizada -> Nullable<Bool>,
        }
    }
}

pub mod models {
    use super::schema::tareas;

    #[derive(Queryable)]
    #[derive(Identifiable)] 
    pub struct Tarea {
        pub id: i32,
        pub tarea: String,
        pub realizada: Option<bool>
    }

    #[derive(Insertable)]
    #[table_name="tareas"]
    pub struct NuevaTarea<'a> {
        pub tarea: &'a str
    }
}
