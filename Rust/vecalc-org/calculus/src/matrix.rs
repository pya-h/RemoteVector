use crate::vector::Vector;

#[derive(Clone, PartialEq)]
pub struct Matrix {
    rows: Vec<Vector>,
    name: String,
}

macro_rules! _M {
        ($($x:expr), *) => {
            {
                let mut m: Matrix {name: "nothing".to_string(), rows: Vec::new()};
                $(
                    m.rows.push($x.clone());
                )*
                m
            }
        };
}

impl Matrix {
    pub fn new(name: &String, rows: Vec<Vector>) -> Matrix {
        Matrix {
            name: name.clone(),
            rows,
        }
    }

    pub fn extend(&mut self, row: &Vector) {
        self.rows.push(row.clone());
    }

    pub fn zeros(n: usize) -> Matrix {
        Matrix {
            name: String::from("Z"),
            rows: (0..n)
                .map(|i| Vector::new(&format!("Z_{}", i), (0..n).map(|_| 0.0_f64).collect()))
                .collect(),
        }
    }
    pub fn to_string(&self) -> String {
        let mut representation: String = String::from("[");

        for xi in &self.rows {
            representation += &format!("\n    {}", xi.as_row())
        }
        representation + "\n]"
    }

    pub fn rows_count(&self) -> usize {
        self.rows.len()
    }
}
