pub mod matrix {
    use crate::calculus::vector::vector::Vector;

    pub struct Matrix {
        rows: Vec<Vector>,
        name: String,
    }

    macro_rules! M {
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
        pub fn new(name: &String, rows: Vec<Vector>)  -> Matrix {
            Matrix {name: name.clone(), rows}
        }
    }
}