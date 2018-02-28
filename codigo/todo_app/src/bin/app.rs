extern crate todo_app;
extern crate diesel;

use self::todo_app::*;
use self::models::*;
use self::diesel::prelude::*;
use std::io;


fn main() {
    let conexion = conectar();
     println!("BIENVENIDO\n");
     acciones(&conexion);
}

fn mostrar_tareas(conexion: &PgConnection) {
    println!("\nTAREAS:");
     use todo_app::schema::tareas::dsl::*;
    let resultados = tareas.load::<Tarea>(conexion).expect("Error al cargar tareas");
    for todo in resultados {
        match todo.realizada {
            Some(true) => print!("[x]"),
            Some(false) => print!("[]"),
            None => print!("[]")
        };
        print!(" {}\n", todo.tarea);
    };
     acciones(&conexion);
}

fn acciones(conexion: &PgConnection) {
    println!("---");
    println!("¿Que desea hacer?");
    println!("1) Agregar tarea");
    println!("2) Eliminar tarea");
    println!("3) Realizar tarea");
    println!("4) Ver tareas");
    println!("5) Salir");

    leer_opcion(conexion,false);

    }

fn leer_opcion(conexion: &PgConnection,con_error: bool){
    if con_error {
        println!("Favor de elegir una opción válida");
    }

    use std::io::prelude::*;


    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let mut buffer = [0; 1];

    // read up to 10 bytes
    input.as_bytes().read(&mut buffer);


    let mut opcion: u8 = 0;

    match buffer.get(0) {
        Some(x) => opcion = x-48,
        None => leer_opcion(conexion,true)
    }

    if opcion == 1 {
        agregar_tarea(conexion);
    } else if opcion == 2 {
        eliminar_tarea(conexion);
    } else if opcion == 3 {
        realizar_tarea(conexion);

    } else if opcion == 4 {
        mostrar_tareas(conexion);
    } else if opcion == 5 {
        println!("\nAdios");

    } else {
        leer_opcion(conexion,true);
    }

}


fn agregar_tarea(conexion: &PgConnection) {
    println!("Tarea a agregar:");
    let mut tarea = String::new();
    io::stdin().read_line(&mut tarea);
    let len = tarea.len();
     tarea.truncate(len - 1);
     let tarea = crear_tarea(conexion,&tarea);
     println!("\nSe agrego la tarea {} con éxito\n", tarea.tarea);
     acciones(conexion);
}

fn eliminar_tarea(conexion: &PgConnection) {
    use todo_app::schema::tareas::dsl::*;
    println!("Tarea a eliminar:");
    let mut patron = String::new();
    io::stdin().read_line(&mut patron);
    let len = patron.len();
    patron.truncate(len - 1);
    let eliminada = diesel::delete(tareas.filter(tarea.like(patron)))
       .execute(conexion)
       .expect("Error al eliminar tarea");

   println!("Se elimino la tarea {}", eliminada);
   acciones(conexion);
}

fn realizar_tarea(conexion: &PgConnection) {
    use todo_app::schema::tareas::dsl::*;
    use diesel::expression::dsl::now;

    println!("Tarea realizada:");
    let mut patron = String::new();
    io::stdin().read_line(&mut patron);
    let len = patron.len();
    patron.truncate(len - 1);

    let filtro = tareas.filter(tarea.eq(patron));
    let post = diesel::update(filtro)
            .set(realizada.eq(true))
            .get_result::<Tarea>(conexion)
            .expect(("No se pudo realizar la tarea"));
        println!("Tarea realizada");

    acciones(conexion);
}
