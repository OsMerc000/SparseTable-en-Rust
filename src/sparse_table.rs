pub use std::fmt::Display;

//Un SparseTable lo podemos construir como un vector de vectores
#[derive(Debug)]
pub struct SparseTable<T> {
    pub st: Vec<Vec<T>>
}

/*
Métodos para el SparseTable
La parte de T: PartialOrd implica que el tipo genérico T es un tipo que podamos 
comparar (<, >, ==, <=, >=) y también clonar.
*/
impl<T: PartialOrd + Clone> SparseTable<T> {
    pub fn new(arr: &[T]) -> Self {
        let len = arr.len();

        /*
        El SparseTable tendrá niveles desde 0 hasta el logaritmo entero de la
        cantidad de elementos del array. Los niveles representan el elemento 
        mínimo del array desde la posición j hasta la posición j + (2^i) - 1
        donde i es el nivel en el que nos encontramos. O sea, si tuvieramos un 
        arr de la siguiente forma:
        
        arr = [1, 6, 7, 8, 2]

        El nivel 0 sería el mismo array.
        
        El nivel 1 sería un array de los elementos mínimos de cada subarray de
        tamaño 2. Tal que así:
        arr1 = [min(0..1), min(1..2), min(2..3), min(3..4)]
        arr1 = [1, 6, 7, 2]

        Y el nivel 2 sería el array de elementos mínimos de cada subarray de
        tamaño 4. Tal que así:
        arr2 = [min(0..3), min(1..4)]
        arr2 = [1, 2]

        Nótese que, debido a lo que representa cada nivel, no puede exirtir un
        nivel 3. Porque eso significaría un subarray de 8 elementos, pero 
        nuestro array solo tiene 5 elementos. Por ende, podemos concluir que 
        la cantidad de niveles máximo el logartimo en base 2 redondeado abajo 
        de la cantidad de elementos del array.
        */
        let size = if arr.len() > 0 {
            arr.len().ilog2()
        } else {
            //Si el array está vacío, el SparTable también lo está.
            return Self {
                st: Vec::new()
            };
        };

        /*
        Alojamos la cantidad suficiente de espacio para los punteros a los
        vectores que conformarán nuestro SparseTable. La razón por la que es
        size + 1 es porque tenemos que incluir el nivel 0 (El mismo array).
        */
        let mut st: Vec<Vec<T>> = Vec::with_capacity(size as usize + 1);
        //Alojamos al nivel 0 el mismo array
        st.push(arr.to_vec());
        //Nos ubicamos en el nivel correspondiente. Size es incusivo.
        for i in 1..=size {
            /*
            Calculamos la cantidad de elementos que habrán en el i-ésimo nivel 
            del SparseTable. Y luego de ello alojamos el espacio necesario.
            */
            let num_elem = len - (2_usize.pow(i) - 1);
            st.push(Vec::with_capacity(num_elem));
            /*
            Esta parte es interesante :).
            
            Nos ubicamos en la posición j del nivel i. Sabemos que este
            elemento representa el mínimo elemento entre el rango j, hasta la 
            posición j + (2^i) - 1. Por ende, en vez de tardar buscando el 
            mínimo elemento en el array, lo que haremos es consultar el nivel 
            anterior del SparseTable. Ya que este ya tiene definido el mínimo 
            elemento u del rango j hasta j + (2^(i-1)) - 1 y el mínimo 
            elemento v del rango j + (2^(i-1)) hasta 
            j + (2^(i-1)) + (2^(i-1)) - 1. Nótese que esta última expresión se
            puede simplificar, tal que su tope es j + (2^i) - 1. O sea, que si
            comparamos u y v, sería los mismo buscar el elemento mínimo desde 
            j hasta j + (2^i) - 1. En otras palabras, nuestra misión es buscar 
            el elemento u y v.
            
            Sabemos que el índice de cada elemento en cada nivel representa a
            su vez el mínimo índice del rango al que pertenece. O sea, el
            índice del elemento que representa al elemento mínimo en el rango
            j..j + (2^i) - 1 es igual a j. Esto se da sin importar el nivel.
            Por tanto, el índice de u será el mismo al del índice donde nos
            encontramos. Para hallar v, sabiendo que su rango inicia en
            j + (2^(i-1)) aplicamos la misma propiedad. Por ello, los
            elementos u y v son aquellos ubicados en el nivel i-1 y cuyos
            índices son j y j + (2^(i-1)) respectivamente.

            Ahora solo debemos de comparar cuál de esos 2 es menor y sabremos
            cuál es el mínimo elemento en el rango j hasta j + (2^i) - 1.
            */
            for j in 0..num_elem {
                let u = st[i as usize - 1][j].clone();
                let v = st[i as usize - 1][j + 2_usize.pow(i - 1)].clone();
                let min = if u < v {u} else {v};
                st[i as usize].push(min);
            }
        }

        //Retornamos el SparseTable
        Self {st}
    }

    pub fn query(&self, left: usize, right: usize) -> Option<&T> {
        if left > right
            || self.st.len() == 0
            || self.st[0].len() <= left 
            || self.st[0].len() <= right {
            return None
        };
        let size = right - left;
        let size = match size.checked_ilog2() {
            None => 0,
            Some(num) => num,
        };
        let u = &self.st[size as usize][left];
        let v = &self.st[size as usize][right - (2_usize.pow(size) - 1)];
        return if u < v {Some(u)} else {Some(v)};
    }
}

impl<T: Display> Display for SparseTable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("");
        for i in &self.st {
            s += "[ ";
            for j in i {
                s = s + &j.to_string() + ", ";
            }
            s.pop();
            s.pop();
            s += " ],\n";
        };
        s.pop();
        s.pop();
        write!(f, "{s}")
    }
}

impl<T: PartialOrd> PartialEq for SparseTable<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.st.len() != other.st.len() {
            return false;
        };

        for i in 0..self.st.len() {
            if self.st[i].len() != other.st[i].len() {
                return false;
            }
            for j in 0..self.st[i].len() {
                if self.st[i][j] != other.st[i][j] {
                    return false;
                }
            }
        };

        return true;
    }

    fn ne(&self, other: &Self) -> bool {
        !Self::eq(&self, &other)
    }
}