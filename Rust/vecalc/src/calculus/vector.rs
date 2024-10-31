pub mod vector {
    pub struct Vector {
        components: Vec<f64>,
        name: String,
    }

    macro_rules! V {
            ($($x:expr), *) => {
                {
                    let mut v: Vector {name: "nothing".to_string(), components: Vec::new()};
                    $(
                        v.components.push($x);
                    )*
                    v
                }
            };
    }

    impl Vector {
        pub fn new(name: &String, v: Vec<f64>) -> Vector {
            Vector {
                components: v,
                name: name.clone(),
            }
        }

        fn zero(dimension: usize) -> Vector {
            Vector {
                name: "nothing".to_string(),
                components: vec![0.0; dimension],
            }
        }

        fn ones(dimension: usize) -> Vector {
            Vector {
                name: "nothing".to_string(),
                components: vec![1.0; dimension],
            }
        }

        fn eye(one_index: usize, dimension: usize) -> Option<Vector> {
            if one_index > dimension {
                return None;
            }
            let mut v = Vector::zero(dimension);
            v.components[one_index - 1] = 1.0;
            Some(v)
        }

        fn add(&self, v: Vector) -> Option<Vector> {
            let u = Vector::zero(self.components.len());

            Some(u)
        }

        pub fn update(&mut self, new_components: Vec<f64>) {
            self.components = new_components;
        }

        pub fn to_string(&self) -> String {
            let mut representation: String = String::from("(");

            for xi in &self.components {
                representation = format!(
                    "{}{}{}",
                    representation,
                    if representation == "(" { "" } else { ", " },
                    if xi.fract() != 0.0 {
                        xi.to_string()
                    } else {
                        (xi.trunc() as i64).to_string()
                    }
                )
            }
            representation + ")"
        }
    }

    pub fn _2d(x: f64, y: f64) -> Vector {
        Vector::new(&"nothing".to_string(), vec![x, y])
    }
}
