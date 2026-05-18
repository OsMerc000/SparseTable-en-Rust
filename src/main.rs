use sparse_table::SparseTable;

mod sparse_table;

fn main() {
    let arr: Vec<i32> = vec![2, 1, 6, 3, 0, 10, -1];
    let st = SparseTable::new(&arr);
    {
        let st1 = SparseTable {
            st: vec![
                vec![2, 1, 6, 3, 0, 10, -1],
                vec![1, 1, 3, 0, 0, -1],
                vec![1, 0, 0, -1]
            ]
        };
        assert_eq!(st, st1);
    }
    assert_eq!(st.query(2, 2), Some(6));
    assert_eq!(st.query(0, 6), Some(-1));
    assert_eq!(st.query(2, 5), Some(0));
    assert_eq!(st.query(9, 10), None);
}