use std::cmp::Ordering;
use std::collections::{ HashMap, HashSet, VecDeque };

use eframe::wgpu::naga::proc::index;

pub struct ListaOrdenada {
    vector: Vec<i32>,
}
impl ListaOrdenada {
    pub fn init(inicial: Option<Vec<i32>>) -> Self {
        //Option permite que la variable sea Some (algo) o None (no existe)
        match inicial {
            //match encuentra el patron donde si la variable es some realiza dicha operacion, de lo contrario es None
            Some(mut v) => {
                // => le dice al compilador que esta buscando un patron
                v.sort();
                Self { vector: v } //La falta de ; retorna este valor
            }
            None => Self { vector: Vec::new() }, // Retorna una
        }
    }

    pub fn done(self) {} //La falta de &mut (referencia mutable) le da la propiedad a la funcion, al terminar, self se destruye

    pub fn clear(&mut self) {
        self.vector.clear();
    }

    pub fn insert(&mut self, element: i32) {
        //recordar que i32 es int de 32
        self.vector.insert(0, element);
        self.vector.sort();
    }

    pub fn delete(&mut self, element: i32) {
        if let Some(index) = self.vector.iter().position(|&x| x == element) {
            //index es una opcion, Some la comvierte en una variable tangible
            self.vector.remove(index); //x es una variable que cumple el patron x == element, position nos da la posicion, iter itera sobre el arreglo y encuentra la variable
        }
    }

    pub fn member(&mut self, element: i32) -> bool {
        //regresa un bool
        self.vector.contains(&element)
    }

    pub fn print(&mut self) {
        println!("{:#?}", self.vector); // el ! en println! indica que es una macro, {} es la entrada de usuario, {:?} es modo debug y realiza print de el arreglo
    }

    fn union(a_lista: ListaOrdenada, b_lista: ListaOrdenada) -> ListaOrdenada {
        let mut new_vec = 
        let new_list = ListaOrdenada::init(a_lista.vector)
    }

    fn intersection(a_lista: ListaOrdenada, b_lista: ListaOrdenada) -> ListaOrdenada {
        let mut new_vector: Vec<i32> = Vec::new();
        for elemento in &a_lista.vector {
            // Como b_lista está ordenado, .binary_search es ultra rápido O(log n)
            if b_lista.vector.binary_search(elemento).is_ok() {
                new_vector.insert(0, *elemento);
            }
        }
        let new_list = ListaOrdenada::init(Some(new_vector));
        return new_list;
    }
}
