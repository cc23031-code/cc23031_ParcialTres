// ============================================================
// Motor de Gestión de Espacio Aéreo — Árbol AVL
// Fase 1: Código base con documentación de ownership
// ============================================================
/*
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

// --- INSERCIÓN ---

fn insertar(nodo_opt: Option<Box<Nodo>>, vuelo: Vuelo) -> Box<Nodo> {
    let mut nodo = match nodo_opt {
        None => return Box::new(Nodo::nuevo(vuelo)),
        Some(n) => n,
    };

    if vuelo.altitud < nodo.vuelo.altitud {
        // take() libera el hijo para pasarlo por valor a la llamada recursiva
        nodo.izquierdo = Some(insertar(nodo.izquierdo.take(), vuelo));
    } else if vuelo.altitud > nodo.vuelo.altitud {
        nodo.derecho = Some(insertar(nodo.derecho.take(), vuelo));
    } else {
        return nodo; // altitud duplicada, ignorar
    }

    actualizar_altura(&mut nodo);
    let balance = obtener_balance(&nodo);

    // Caso LL — subárbol izquierdo, hijo izquierdo
    if balance > 1 && vuelo.altitud < nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
        return rotar_derecha(nodo);
    }
    // Caso RR — subárbol derecho, hijo derecho
    if balance < -1 && vuelo.altitud > nodo.derecho.as_ref().unwrap().vuelo.altitud {
        return rotar_izquierda(nodo);
    }
    // Caso LR — subárbol izquierdo, hijo derecho
    if balance > 1 && vuelo.altitud > nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
        let hijo_izq = nodo.izquierdo.take().unwrap();
        nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
        return rotar_derecha(nodo);
    }
    // Caso RL — subárbol derecho, hijo izquierdo
    if balance < -1 && vuelo.altitud < nodo.derecho.as_ref().unwrap().vuelo.altitud {
        let hijo_der = nodo.derecho.take().unwrap();
        nodo.derecho = Some(rotar_derecha(hijo_der));
        return rotar_izquierda(nodo);
    }

    nodo
}

fn main() {
    let mut radar: Option<Box<Nodo>> = None;

    let datos = vec![
        ("AV123", 5000), ("UA456", 3000), ("IB101", 2000),
        ("AF999", 4000), ("TA222", 3500), ("AM777", 6000),
    ];

    for (id, alt) in datos {
        let v = Vuelo { id: id.to_string(), altitud: alt };
        radar = Some(insertar(radar.take(), v));
    }

    println!("--- Radar de Control Aéreo (AVL) ---");
    println!("Árbol construido con {} vuelos.", 6);
    // Las fases 2, 3 y 4 se invocarán aquí
}
*/
