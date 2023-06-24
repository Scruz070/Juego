use std::io;
// Importar el módulo jugador.rs
mod jugador; 

// Importar los elementos necesarios del módulo jugador.rs
use jugador::{JugadorNBA, crear_jugador, imprimir_jugador, buscar_jugador, buscar_jugador_mas_puntos}; 

fn main() { 
    let jugador1 = crear_jugador(String::from("LeBron James"),
    String::from("Alero"),
    String::from("Los Angeles Lakers"),
    60, 34.7, 25.0);

    let jugador2 = crear_jugador(String::from("Stephen Curry"),
    String::from("Base"),
    String::from("Golden State Warriors"),
    58, 33.2, 29.5);

    let jugador3 = crear_jugador(String::from("Kevin Durant"),
    String::from("Alero"),
    String::from("Brooklyn Nets"),
    45,35.6, 27.3);

    let jugador4 = crear_jugador(String::from("Giannis Antetokounmpo"),
    String::from("Alero"),
    String::from("Milwaukee Bucks"),
    62, 36.1, 28.5);

    let jugador5 = crear_jugador(String::from("Luka Dončić"),
    String::from("Base"),
    String::from("Dallas Mavericks"),
    56, 35.2, 26.9);        

    // Crear un arreglo de JugadorNBA
    let arreglo_jugadores = [jugador1, jugador2, jugador3, jugador4, jugador5];

    loop {
        println!("\n\nMenú de la aplicación");
        println!("---------------------");
        println!("1. Listar todos los jugadores");
        println!("2. Buscar jugador por nombre");
        println!("3. Jugador con mejor promedio de puntos");
        println!("4. Salir");
        println!("Ingrese una opción:");

        let mut opcion = String::new();
        io::stdin()
            .read_line(&mut opcion)
            .expect("Error al leer la entrada");

        match opcion.trim() {
            "1" => {                
                println!("\n-> Jugadores registrados en la aplicación");
                for jugador in &arreglo_jugadores {
                    imprimir_jugador(&jugador);
                }
            }
            "2" => {                
                println!("\n-> Buscar jugador por nombre");                
                let nombre_buscar = String::from("Giannis Antetokounmpo");         
                let jugador_encontrado = buscar_jugador(&arreglo_jugadores, &nombre_buscar.trim());

                match jugador_encontrado {
                    Some(jugador) => {
                    println!("\n*** Jugador encontrado: ***");
                    imprimir_jugador(jugador);
                },
                None => println!("\n -> No se encontró ningún jugador con ese nombre."),
                }
            }
            "3" => {
                if let Some(jugador) = buscar_jugador_mas_puntos(&arreglo_jugadores) {
                    println!("\n-> Jugador con más puntos por juego:");
                    imprimir_jugador(jugador);
                } else {
                    println!("\n -> No se encontró ningún jugador.");
                }
            }
            "4" => {
                println!("*** Saliendo de la aplicación *** ");
                break;
            }
            _ => {
                println!("-> Opción inválida. Por favor, ingrese una opción válida.");
            }
        }
    }
}