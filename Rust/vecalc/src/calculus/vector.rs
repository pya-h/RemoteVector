use crate::calculus::matrix::Matrix;

#[derive(Clone)]
pub struct Vector {
    components: Vec<f64>,
    name: String,
}

#[macro_export] macro_rules! V {
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
        u.name = format!(
            "{}{}{}",
            self.name,
            if c != 1.0 {
                if c != -1.0 {
                    String::from(" + ") + &c.to_string()
                } else {
                    String::from(" - ")
                }
            } else {
                String::from(" + ")
            },
            v.name
        );
        Some(u)
    }

    pub fn add(&self, v: &Vector) -> Option<Vector> {
        self.plus_cv(v, 1.0)
    }

    pub fn sub(&self, v: &Vector) -> Option<Vector> {
        self.plus_cv(v, -1.0)
    }

    pub fn dot(&self, v: &Vector) -> Option<f64> {
        let n = self.components.len();
        if n != v.components.len() {
            return None;
        }
        let mut u: f64 = 0.0;
        for i in 0..n {
            u += self.components[i] * v.components[i];
        }
        Some(u)
    }

    pub fn multiply(&self, v: &Vector) -> Option<Vector> {
        let n = self.components.len();
        if n != v.components.len() {
            return None;
        }
        let mut u: Vector = Vector::zero(n);
        for i in 0..n {
            u.components[i] = self.components[i] * v.components[i];
        }
        u.name = format!("{} * 1{}", self.name, v.name);
        Some(u)
    }

    pub fn cross(&self, v: &Vector) -> Matrix {
        let mut outer_product: Matrix =
            Matrix::new(&format!("{} x {}", self.name, v.name), Vec::new());
        for x in &self.components {
            outer_product.extend(&v.map(*x, 0.0))
        }
        outer_product
    }

    pub fn map(&self, multiply_by: f64, increment_by: f64) -> Vector {
        let mut u: Vector = Vector::zero(self.components.len());
        u.name = format!(
            "{}{}{}",
            if multiply_by != 1.0 {
                if multiply_by != -1.0 {
                    multiply_by.to_string()
                } else {
                    String::from("-")
                }
            } else {
                String::from("")
            },
            self.name,
            if increment_by != 0.0 {
                format!("+ {}", increment_by)
            } else {
                String::from("")
            }
        );
        for (i, &xi) in self.components.iter().enumerate() {
            u.components[i] = multiply_by * xi + increment_by;
        }
        u
    }

    pub fn update(&mut self, new_components: Vec<f64>) {
        self.components = new_components;
    }

    pub fn as_row(&self) -> String {
        let mut representation: String = String::new();

        for xi in &self.components {
            representation = format!(
                "{}{}{}",
                representation,
                if representation.is_empty() { "" } else { ", " },
                if xi.fract() != 0.0 {
                    xi.to_string()
                } else {
                    (xi.trunc() as i64).to_string()
                }
            )
        }
        representation
    }

    pub fn to_string(&self) -> String {
        format!("({})", self.as_row())
    }

    pub fn definition_string(&self) -> String {
        format!("{} = {}", self.name, self.to_string())
    }

    pub fn clone(&self) -> Self {
        Vector::new(&format!("{}_copy", self.name), self.components.clone())
    }
}

pub fn _2d(x: f64, y: f64) -> Vector {
    Vector::new(&"nothing".to_string(), vec![x, y])
}
