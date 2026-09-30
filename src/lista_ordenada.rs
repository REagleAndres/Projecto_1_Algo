use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Write;

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

    pub fn print_to<W: Write>(&self, mut dest: W) {
        //esta funcion se usa solo para test
        // writeln! funciona igual que println!, pero escribe en el destino que le digas
        writeln!(dest, "{:#?}", self.vector).unwrap();
    }

    fn union(a_lista: &ListaOrdenada, b_lista: &ListaOrdenada) -> ListaOrdenada {
        let new_vec: Vec<i32> = [a_lista.vector.as_slice(), b_lista.vector.as_slice()].concat(); //as_slice devuelve una lista de lectura sin modificar la original
        let new_list = ListaOrdenada::init(Some(new_vec)); //concat crea una nueva lista igual a ambas, el [a,b] nos da un arreglo temporal
        return new_list;
    }

    fn intersection(a_lista: &ListaOrdenada, b_lista: &ListaOrdenada) -> ListaOrdenada {
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

    fn difference(a_lista: &ListaOrdenada, b_lista: &ListaOrdenada) -> ListaOrdenada {
        let mut new_vector: Vec<i32> = Vec::new();
        for elemento in &a_lista.vector {
            //igual que el anterior pero agregamos aquellos que no estan en b
            if b_lista.vector.binary_search(elemento).is_err() {
                new_vector.insert(0, *elemento);
            }
        }
        for elemento in &b_lista.vector {
            //aqui agregamos aquellos que no estan en b
            if a_lista.vector.binary_search(elemento).is_err() {
                new_vector.insert(0, *elemento);
            }
        }
        let new_list = ListaOrdenada::init(Some(new_vector));
        return new_list;
    }
}

#[cfg(test)] //indica al compilador que lo que sigue es tests
mod tests {
    use super::*; //trae todas las funciones del modulo anterior
    //let a_list = ListaOrdenada::init(vec![0, 1, 2, 3, 5, 8, 13]);

    #[test]
    fn init_test() {
        let a_list = ListaOrdenada::init(Some(vec![0, 1, 2, 3, 5, 8, 13]));
        let b_list = ListaOrdenada::init(None);

        assert_eq!(a_list.vector, vec![0, 1, 2, 3, 5, 8, 13]);
        assert_eq!(b_list.vector, vec![]);
    }

    #[test]
    fn clear_test() {
        let mut a_list = ListaOrdenada::init(Some(vec![0, 1, 2, 3, 5, 8, 13]));
        a_list.clear();
        assert_eq!(a_list.vector, vec![]);
    }

    #[test]
    fn insert_test() {
        let mut a_list = ListaOrdenada::init(Some(vec![0, 1, 2, 3, 5, 8, 13]));
        a_list.insert(4);
        assert_eq!(a_list.vector, vec![0, 1, 2, 3, 4, 5, 8, 13]);
    }

    #[test]
    fn delete_test() {
        let mut a_list = ListaOrdenada::init(Some(vec![0, 1, 2, 3, 5, 8, 13]));
        a_list.delete(5);
        assert_eq!(a_list.vector, vec![0, 1, 2, 3, 8, 13]);
        a_list.delete(4);
        assert_eq!(a_list.vector, vec![0, 1, 2, 3, 8, 13]);
    }

    #[test]
    fn member_test() {
        let mut a_list = ListaOrdenada::init(Some(vec![0, 1, 2, 3, 5, 8, 13]));
        assert!(a_list.member(5));
        assert!(!a_list.member(4));
    }

    #[test]
    fn print_test() {
        let a_list = ListaOrdenada {
            vector: vec![1, 2, 3],
        };
        // Creamos un buffer en memoria (un vector de bytes) en lugar de la consola
        let mut buffer = Vec::new();
        // Llamamos a la función apuntando a nuestro buffer
        a_list.print_to(&mut buffer);
        // Convertimos los bytes capturados a un String legible
        let resultado = String::from_utf8(buffer).unwrap();
        // El assert_eq! con el formato exacto de {:#?} que genera un Vec
        let esperado = "[\n    1,\n    2,\n    3,\n]\n";
        assert_eq!(resultado, esperado);
    }

    #[test]
    fn union_test() {
        let a_list = ListaOrdenada::init(Some(vec![0, 1, 2, 3, 5, 8, 13]));
        let b_list = ListaOrdenada::init(Some(vec![4, 6, 7, 9, 10, 11, 12]));

        assert_eq!(
            ListaOrdenada::union(&a_list, &b_list).vector,
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]
        );
    }

    #[test]
    fn intersection_test() {
        let a_list = ListaOrdenada::init(Some(vec![0, 1, 2, 3, 5, 8, 13]));
        let b_list = ListaOrdenada::init(Some(vec![4, 6, 7, 9, 10, 11, 12]));
        let c_list = ListaOrdenada::init(Some(vec![0, 1, 2, 2, 4, 8, 32]));

        assert_eq!(ListaOrdenada::intersection(&a_list, &b_list).vector, vec![]);
        assert_eq!(
            ListaOrdenada::intersection(&a_list, &c_list).vector,
            vec![0, 1, 2, 8]
        );
    }

    #[test]
    fn difference_test() {
        let a_list = ListaOrdenada::init(Some(vec![0, 1, 2, 3, 5, 8, 13]));
        let b_list = ListaOrdenada::init(Some(vec![4, 6, 7, 9, 10, 11, 12]));
        let c_list = ListaOrdenada::init(Some(vec![0, 1, 2, 2, 4, 8, 32]));

        assert_eq!(
            ListaOrdenada::difference(&a_list, &b_list).vector,
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]
        );
        assert_eq!(
            ListaOrdenada::difference(&a_list, &c_list).vector,
            vec![3, 4, 5, 13, 32]
        );
    }
}
