pub mod vector {
    #[derive(Clone)]
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

        pub fn zero(dimension: usize) -> Vector {
            Vector {
                name: "nothing".to_string(),
                components: vec![0.0; dimension],
            }
        }

        pub fn ones(dimension: usize) -> Vector {
            Vector {
                name: "nothing".to_string(),
                components: vec![1.0; dimension],
            }
        }

        pub fn eye(one_index: usize, dimension: usize) -> Option<Vector> {
            if one_index > dimension {
                return None;
            }
            let mut v = Vector::zero(dimension);
            v.components[one_index - 1] = 1.0;
            Some(v)
        }
        
        pub fn plus_cv(&self, v: &Vector, c: f64) -> Option<Vector> {
            let n = self.components.len();
            if n != v.components.len() {
                return None;
            }
            let mut u: Vector = Vector::zero(n);
            for i in 0..n {
                u.components[i] = self.components[i] + c * v.components[i];
            }
            Some(u)
        }

        pub fn add(&self, v: &Vector) -> Option<Vector> {
           self.plus_cv(v, 1.0)
        }

        pub fn sub(&self, v: &Vector) -> Option<Vector> {
            self.plus_cv(v, -1.0)
        }
    
        pub fn dot(&self, v: &Vector) -> Option<Vector> {
            let n = self.components.len();
            if n != v.components.len() {
                return None;
            }
            let mut u: Vector = Vector::zero(n);
            for i in 0..n {
                u.components[i] = self.components[i] * v.components[i];
            }
            Some(u)
        }

        pub fn map(&self, multiply_by: f64, increment_by: f64) -> Vector {
            let mut u: Vector = Vector::zero(self.components.len());
            for (i, &xi) in self.components.iter().enumerate() {
                u.components[i] = multiply_by*xi + increment_by;
            }
            u
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

        pub fn clone(&self) -> Self {
            Vector::new(&format!("{}_copy", self.name), self.components.clone())
        }
    }

    pub fn _2d(x: f64, y: f64) -> Vector {
        Vector::new(&"nothing".to_string(), vec![x, y])
    }
}
