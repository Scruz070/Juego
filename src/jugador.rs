pub struct JugadorNBA {
    nombre: String,
    posicion: String,
    equipo: String,    
    partidos_jugados: i8,
    minutos_por_juego: f32,
    puntos_por_juego: f32,
}

pub fn crear_jugador(nombre: String, posicion: String, equipo: String, partidos_jugados: i8, 
                 minutos_por_juego: f32, puntos_por_juego: f32) -> JugadorNBA {
    JugadorNBA {
        nombre,
        posicion,
        equipo,    
        partidos_jugados,
        minutos_por_juego,
        puntos_por_juego,
    }
}

pub fn imprimir_jugador(jugador: &JugadorNBA) {
    println!("\n*** Información del jugador ***");
    println!("Nombre: {}", jugador.nombre);
    println!("Equipo: {}", jugador.equipo);
    println!("Posición: {}", jugador.posicion);    
    println!("Total partidos jugados: {}", jugador.partidos_jugados);
    println!("Promedio de minutos por partido: {}", jugador.minutos_por_juego);
    println!("Promedio de puntos por partido: {}", jugador.puntos_por_juego);
}

pub fn buscar_jugador<'a>(jugadores: &'a [JugadorNBA], nombre_buscar: &str) -> Option<&'a JugadorNBA> {
    for jugador in jugadores {
        if jugador.nombre == nombre_buscar {
            return Some(jugador);
        }
    }
    None
}

pub fn buscar_jugador_mas_puntos(jugadores: &[JugadorNBA]) -> Option<&JugadorNBA> {
    let mut max_puntos_por_juego = 0.0;
    let mut jugador_mas_puntos: Option<&JugadorNBA> = None;

    for jugador in jugadores {
        if jugador.puntos_por_juego > max_puntos_por_juego {
            max_puntos_por_juego = jugador.puntos_por_juego;
            jugador_mas_puntos = Some(jugador);
        }
    }

    jugador_mas_puntos
}
