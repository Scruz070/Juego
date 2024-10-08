// Definición de la estructura `JugadorNBA` que representa a un jugador de la NBA
pub struct JugadorNBA {
    nombre: String,              // Nombre del jugador
    posicion: String,            // Posición en la que juega
    equipo: String,              // Equipo en el que juega
    partidos_jugados: i8,        // Número de partidos jugados
    minutos_por_juego: f32,      // Promedio de minutos jugados por partido
    puntos_por_juego: f32,       // Promedio de puntos anotados por partido
}

// Función que crea y retorna un nuevo jugador de la NBA
pub fn crear_jugador(
    nombre: String, 
    posicion: String, 
    equipo: String, 
    partidos_jugados: i8, 
    minutos_por_juego: f32, 
    puntos_por_juego: f32
) -> JugadorNBA {
    JugadorNBA {
        nombre,
        posicion,
        equipo,    
        partidos_jugados,
        minutos_por_juego,
        puntos_por_juego,
    }
}

// Función que imprime los detalles de un jugador dado
pub fn imprimir_jugador(jugador: &JugadorNBA) {
    println!("\n*** Información del jugador ***");
    println!("Nombre: {}", jugador.nombre);                   // Imprime el nombre del jugador
    println!("Equipo: {}", jugador.equipo);                   // Imprime el equipo
    println!("Posición: {}", jugador.posicion);               // Imprime la posición
    println!("Total partidos jugados: {}", jugador.partidos_jugados);  // Imprime la cantidad de partidos jugados
    println!("Promedio de minutos por partido: {}", jugador.minutos_por_juego);  // Imprime el promedio de minutos por partido
    println!("Promedio de puntos por partido: {}", jugador.puntos_por_juego);    // Imprime el promedio de puntos por partido
}

// Función que busca un jugador en una lista por su nombre
pub fn buscar_jugador<'a>(jugadores: &'a [JugadorNBA], nombre_buscar: &str) -> Option<&'a JugadorNBA> {
    // Itera sobre los jugadores en la lista
    for jugador in jugadores {
        // Si encuentra un jugador con el nombre buscado, lo retorna
        if jugador.nombre == nombre_buscar {
            return Some(jugador);
        }
    }
    None  // Si no lo encuentra, retorna `None`
}

// Función que busca el jugador con más puntos por partido
pub fn buscar_jugador_mas_puntos(jugadores: &[JugadorNBA]) -> Option<&JugadorNBA> {
    let mut max_puntos_por_juego = 0.0;                // Inicializa el máximo de puntos con 0
    let mut jugador_mas_puntos: Option<&JugadorNBA> = None;  // Inicializa la variable del jugador con más puntos

    // Itera sobre todos los jugadores
    for jugador in jugadores {
        // Si encuentra un jugador con un promedio de puntos mayor al máximo actual
        if jugador.puntos_por_juego > max_puntos_por_juego {
            max_puntos_por_juego = jugador.puntos_por_juego;  // Actualiza el máximo de puntos
            jugador_mas_puntos = Some(jugador);               // Actualiza el jugador con más puntos
        }
    }

    jugador_mas_puntos  // Retorna el jugador con más puntos, o `None` si no hay jugadores
}
