pub use std::fmt::Display;

pub struct SparseTable<T> {
    st: Vec<Vec<T>>
}

impl<T: PartialOrd + Clone> SparseTable<T> {
    pub fn new(arr: &[T]) -> Self {
        let len = arr.len();
        let size = match len.checked_ilog2() {
            Some(num) => num,
            None => {
                return Self {
                    st: Vec::new()
                }
            }
        };

        let mut st: Vec<Vec<T>> = Vec::with_capacity(size as usize + 1);
        st.push(arr.to_vec());
        for i in 1..=size {
            let num_elem = len - (2_usize.pow(i) - 1);
            st.push(Vec::with_capacity(num_elem));
            for j in 0..num_elem {
                let v = st[i as usize - 1][j].clone();
                let u = st[i as usize - 1][j + 2_usize.pow(i - 1)].clone();
                let min = if v < u {v} else {u};
                st[i as usize].push(min);
            }
        }

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