use sparse_table::{SparseTable, Display};

mod sparse_table;

fn main() {
    let arr: Vec<i32> = vec![2, 1, 6, 3, 0, 10, -1];
    print!("Este es su array: ");
    show_arr(&arr);
    let st = SparseTable::new(&arr);
    println!("\nEste es su SparseTable = {{\n{st}\n}}");
    print!("Query: 2 - 2 = ");
    if let Some(num) = st.query(2, 2) {
        println!("{num}")
    } else {
        println!("None")
    }
    print!("Query: 0 - 6 = ");
    if let Some(num) = st.query(0, 6) {
        println!("{num}")
    } else {
        println!("None")
    }
    print!("Query: 2 - 5 = ");
    if let Some(num) = st.query(2, 5) {
        println!("{num}")
    } else {
        println!("None")
    }
}

fn show_arr<T: Display>(arr: &[T]) {
    let mut s = String::new();
    for e in arr {
        s = s + &e.to_string() + ", ";
    }
    s.pop();
    s.pop();
    print!("[ {} ]", s);
}