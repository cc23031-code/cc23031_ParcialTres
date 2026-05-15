// ============================================================
// Motor de Gestión de Espacio Aéreo — Árbol AVL
// Fase 1: Código base con documentación de ownership
// ============================================================
/*
# Motor de Tráfico Aéreo - AVL

FASE 1: Prueba de Escritorio
Inserciones: [5000, 3000, 2000, 4000, 3500, 6000]

 Paso 1: Insertar 5000
        5000
        (árbol balanceado, balance=0)

 Paso 2: Insertar 3000
        5000
       /
     3000
     (balance=1, )

 Paso 3: Insertar 2000
        5000
       /
     3000
     /
   2000
   (balance=2 en 5000  → ROTACIÓN SIMPLE DERECHA)
   Resultado:
        3000
       /    \
     2000   5000

 Paso 4: Insertar 4000
        3000
       /    \
     2000   5000
            /
          4000
          (balance=-1 en 3000, )

 Paso 5: Insertar 3500
        3000
       /    \
     2000   5000
            /
          4000
          /
        3500
        (balance=-2 en 3000  → ROTACIÓN DOBLE Derecha-Izquierda)
        Primero rotación derecha en 5000, luego izquierda en 3000
   Resultado:
          3500
         /    \
       3000   5000
       /      /
     2000   4000

 Paso 6: Insertar 6000
          3500
         /    \
       3000   5000
       /      /  \
     2000   4000  6000
     (balance=0, árbol balanceado )

## Rotaciones identificadas:
- Paso 3 → Rotación SIMPLE DERECHA (caso Izquierda-Izquierda)
- Paso 5 → Rotación DOBLE Derecha-Izquierda (caso Derecha-Izquierda)
*/

#[derive(Debug, Clone)]
struct Vuelo {
    id: String,
    altitud: u32, // Clave del árbol AVL
}

struct Nodo {
    vuelo: Vuelo,
    izquierdo: Option<Box<Nodo>>,
    derecho: Option<Box<Nodo>>,
    altura: i32,
}

impl Nodo {
    fn nuevo(vuelo: Vuelo) -> Self {
        Nodo {
            vuelo,
            izquierdo: None,
            derecho: None,
            altura: 1,
        }
    }
}

// --- UTILIDADES DE BALANCEO (NO MODIFICAR) ---

fn obtener_altura(nodo: &Option<Box<Nodo>>) -> i32 {
    // as_ref() presta el contenido del Option sin moverlo
    nodo.as_ref().map_or(0, |n| n.altura)
}

fn actualizar_altura(nodo: &mut Nodo) {
    nodo.altura = 1 + std::cmp::max(
        obtener_altura(&nodo.izquierdo),
        obtener_altura(&nodo.derecho),
    );
}

fn obtener_balance(nodo: &Nodo) -> i32 {
    obtener_altura(&nodo.izquierdo) - obtener_altura(&nodo.derecho)
}

// FASE 1 — OWNERSHIP EN ROTACIONES:
// take() transfiere ownership del hijo al scope local, dejando None en su lugar.
// Esto evita tener dos referencias mutables al mismo dato simultáneamente,
// lo cual el borrow checker de Rust prohíbe en tiempo de compilación.
fn rotar_derecha(mut y: Box<Nodo>) -> Box<Nodo> {
    // take() mueve y.izquierdo a x; y.izquierdo queda en None
    let mut x = y.izquierdo.take().expect("Error de radar");
    // take() mueve x.derecho a y.izquierdo; x.derecho queda en None
    y.izquierdo = x.derecho.take();
    actualizar_altura(&mut y);
    x.derecho = Some(y);
    actualizar_altura(&mut x);
    x // x es ahora la nueva raíz del subárbol
}

fn rotar_izquierda(mut x: Box<Nodo>) -> Box<Nodo> {
    let mut y = x.derecho.take().expect("Error de radar");
    x.derecho = y.izquierdo.take();
    actualizar_altura(&mut x);
    y.izquierdo = Some(x);
    actualizar_altura(&mut y);
    y
}
/// FASE 2: Localización de Vuelos
/// Busca un vuelo específico por su altitud.
/// Retorna una referencia al vuelo si existe, garantizando que sea de solo lectura.
fn buscar_vuelo(nodo: &Option<Box<Nodo>>, altitud: u32) -> Option<&Vuelo> {
    // .as_ref() convierte el Option<Box<Nodo>> en Option<&Box<Nodo>>
    // permitiéndonos leer el contenido sin tomar posesión (ownership).
    match nodo.as_ref() {
        None => None, // No se encontró ningún vuelo a esa altitud
        Some(n) => {
            if altitud == n.vuelo.altitud {
                Some(&n.vuelo) // Vuelo encontrado, retornamos la referencia
            } else if altitud < n.vuelo.altitud {
                // Si la altitud buscada es menor, buscamos en el subárbol izquierdo
                buscar_vuelo(&n.izquierdo, altitud)
            } else {
                // Si es mayor, buscamos en el subárbol derecho
                buscar_vuelo(&n.derecho, altitud)
            }
        }
    }
}

// --- INSERCIÓN ---

fn insertar(nodo_opt: Option<Box<Nodo>>, vuelo: Vuelo) -> Box<Nodo> {
    let mut nodo = match nodo_opt {
        None => return Box::new(Nodo::nuevo(vuelo)),
        Some(n) => n,
    };

    // GUARDAMOS LA ALTITUD antes de que 'vuelo' se mueva
    let altitud_vuelo = vuelo.altitud;

    if altitud_vuelo < nodo.vuelo.altitud {
        // Usamos .clone() para que 'vuelo' siga existiendo para las validaciones de abajo
        nodo.izquierdo = Some(insertar(nodo.izquierdo.take(), vuelo.clone()));
    } else if altitud_vuelo > nodo.vuelo.altitud {
        nodo.derecho = Some(insertar(nodo.derecho.take(), vuelo.clone()));
    } else {
        return nodo;
    }

    actualizar_altura(&mut nodo);
    let balance = obtener_balance(&nodo);

    // --- RE-BALANCEO ---
    // Usamos 'altitud_vuelo' (la copia simple) en lugar de 'vuelo.altitud'

    // Caso Izquierda-Izquierda
    if balance > 1 && altitud_vuelo < nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
        return rotar_derecha(nodo);
    }
    // Caso Derecha-Derecha
    if balance < -1 && altitud_vuelo > nodo.derecho.as_ref().unwrap().vuelo.altitud {
        return rotar_izquierda(nodo);
    }
    // Caso Izquierda-Derecha
    if balance > 1 && altitud_vuelo > nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
        let hijo_izq = nodo.izquierdo.take().unwrap();
        nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
        return rotar_derecha(nodo);
    }
    // Caso Derecha-Izquierda
    if balance < -1 && altitud_vuelo < nodo.derecho.as_ref().unwrap().vuelo.altitud {
        let hijo_der = nodo.derecho.take().unwrap();
        nodo.derecho = Some(rotar_derecha(hijo_der));
        return rotar_izquierda(nodo);
    }

    nodo
}
fn eliminar_vuelo(nodo_opt: Option<Box<Nodo>>, altitud: u32) -> Option<Box<Nodo>> {
    let mut nodo = nodo_opt?; // Si el nodo es None, retorna None (termina la recursión)

    // 1. Búsqueda del nodo a eliminar
    if altitud < nodo.vuelo.altitud {
        nodo.izquierdo = eliminar_vuelo(nodo.izquierdo.take(), altitud);
    } else if altitud > nodo.vuelo.altitud {
        nodo.derecho = eliminar_vuelo(nodo.derecho.take(), altitud);
    } else {
        // --- ¡NODO ENCONTRADO! ---

        // Caso A y B: El nodo tiene un solo hijo o ninguno
        if nodo.izquierdo.is_none() {
            return nodo.derecho;
        } else if nodo.derecho.is_none() {
            return nodo.izquierdo;
        }

        // Caso C: El nodo tiene dos hijos
        // Buscamos el "predecesor in-order" (el más alto del subárbol izquierdo)
        let v_predecesor = {
            let mut temp = nodo.izquierdo.as_ref().unwrap();
            while let Some(ref d) = temp.derecho {
                temp = d;
            }
            temp.vuelo.clone()
        };

        // Copiamos los datos del predecesor al nodo actual
        nodo.vuelo = v_predecesor;
        // Eliminamos el predecesor en el subárbol izquierdo
        nodo.izquierdo = eliminar_vuelo(nodo.izquierdo.take(), nodo.vuelo.altitud);
    }

    // 2. Actualizar altura del nodo actual
    actualizar_altura(&mut nodo);

    // 3. Re-balancear el árbol (Vital para que siga siendo AVL)
    let balance = obtener_balance(&nodo);

    // Caso Izquierda-Izquierda
    if balance > 1 && obtener_balance(nodo.izquierdo.as_ref().unwrap()) >= 0 {
        return Some(rotar_derecha(nodo));
    }

    // Caso Izquierda-Derecha
    if balance > 1 && obtener_balance(nodo.izquierdo.as_ref().unwrap()) < 0 {
        let hijo_izq = nodo.izquierdo.take().unwrap();
        nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
        return Some(rotar_derecha(nodo));
    }

    // Caso Derecha-Derecha
    if balance < -1 && obtener_balance(nodo.derecho.as_ref().unwrap()) <= 0 {
        return Some(rotar_izquierda(nodo));
    }

    // Caso Derecha-Izquierda
    if balance < -1 && obtener_balance(nodo.derecho.as_ref().unwrap()) > 0 {
        let hijo_der = nodo.derecho.take().unwrap();
        nodo.derecho = Some(rotar_derecha(hijo_der));
        return Some(rotar_izquierda(nodo));
    }

    Some(nodo)
}

fn main() {
    let mut radar: Option<Box<Nodo>> = None;

    let datos = vec![
        ("AV123", 5000),
        ("UA456", 3000),
        ("IB101", 2000),
        ("AF999", 4000),
        ("TA222", 3500),
        ("AM777", 6000),
    ];

    for (id, alt) in datos {
        let v = Vuelo {
            id: id.to_string(),
            altitud: alt,
        };
        radar = Some(insertar(radar.take(), v));
    }

    println!("--- Radar de Control Aéreo (AVL) ---");
    println!("Árbol construido con {} vuelos.", 6);
    // Las fases 2, 3 y 4 se invocarán aquí
    // --- PRUEBA FASE 2 ---
    println!("\n--- Verificación de Localización ---");
    let altitud_objetivo = 4000;
    match buscar_vuelo(&radar, altitud_objetivo) {
        Some(v) => println!("Radar: Vuelo {} identificado a {} pies.", v.id, v.altitud),
        None => println!("Radar: No se detectan vuelos a {} pies.", altitud_objetivo),
    }

    let altitud_falsa = 10000;
    if buscar_vuelo(&radar, altitud_falsa).is_none() {
        println!("Radar: Espacio aéreo despejado a {} pies.", altitud_falsa);
    }
    println!("\n--- Fase 3: Simulación de Aterrizaje ---");
    let altitud_aterrizaje = 3000;
    println!(
        "Control: Ordenando aterrizaje para el vuelo a {} pies...",
        altitud_aterrizaje
    );

    // Llamada a la eliminación
    radar = eliminar_vuelo(radar.take(), altitud_aterrizaje);

    // Verificación
    match buscar_vuelo(&radar, altitud_aterrizaje) {
        Some(_) => println!("Error: El vuelo aún aparece en el radar."),
        None => println!(
            " Éxito: Espacio aéreo despejado a {} pies.",
            altitud_aterrizaje
        ),
    }
}
