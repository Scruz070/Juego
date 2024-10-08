use std::io;
// Importar el módulo jugador.rs
mod jugador;

// Importar los elementos necesarios del módulo jugador.rs
use jugador::{JugadorNBA, crear_jugador, imprimir_jugador, buscar_jugador, buscar_jugador_mas_puntos};

fn main() { 
    // Crear instancias de JugadorNBA utilizando la función `crear_jugador`
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
        45, 35.6, 27.3);

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

    // Ciclo que maneja el menú de la aplicación
    loop {
        // Mostrar el menú de opciones
        println!("\n\nMenú de la aplicación");
        println!("---------------------");
        println!("1. Listar todos los jugadores");
        println!("2. Buscar jugador por nombre");
        println!("3. Jugador con mejor promedio de puntos");
        println!("4. Salir");
        println!("Ingrese una opción:");

        // Leer la opción del usuario
        let mut opcion = String::new();
        io::stdin()
            .read_line(&mut opcion)
            .expect("Error al leer la entrada");

        // Evaluar la opción ingresada
        match opcion.trim() {
            "1" => {                
                // Opción 1: Listar todos los jugadores
                println!("\n-> Jugadores registrados en la aplicación");
                for jugador in &arreglo_jugadores {
                    imprimir_jugador(&jugador);  // Imprimir los datos de cada jugador
                }
            }
            "2" => {                
                // Opción 2: Buscar un jugador por su nombre
                println!("\n-> Buscar jugador por nombre");
                let nombre_buscar = String::from("Giannis Antetokounmpo");  // Nombre a buscar
                let jugador_encontrado = buscar_jugador(&arreglo_jugadores, &nombre_buscar.trim());

                // Verificar si se encontró el jugador
                match jugador_encontrado {
                    Some(jugador) => {
                        println!("\n*** Jugador encontrado: ***");
                        imprimir_jugador(jugador);  // Imprimir la información del jugador encontrado
                    },
                    None => println!("\n -> No se encontró ningún jugador con ese nombre."),
                }
            }
            "3" => {
                // Opción 3: Buscar el jugador con el mayor promedio de puntos
                if let Some(jugador) = buscar_jugador_mas_puntos(&arreglo_jugadores) {
                    println!("\n-> Jugador con más puntos por juego:");
                    imprimir_jugador(jugador);  // Imprimir los datos del jugador con más puntos
                } else {
                    println!("\n -> No se encontró ningún jugador.");
                }
            }
            "4" => {
                // Opción 4: Salir de la aplicación
                println!("*** Saliendo de la aplicación *** ");
                break;  // Salir del bucle
            }
            _ => {
                // Opción inválida: mostrar mensaje de error
                println!("-> Opción inválida. Por favor, ingrese una opción válida.");
            }
        }
    }
}
