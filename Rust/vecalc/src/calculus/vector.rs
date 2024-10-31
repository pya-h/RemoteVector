
pub mod vector {
    pub struct Vector {
        components: Vec<f64>,
        name: String,
    }

    static mut memory: Vec<Vector> = Vec::new();

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
        fn define(name: &String, v: Vec<f64>) {
            memory.push(Vector {
                components: v,
                name: name.clone(),
            });
        }

        fn zero(dimension: usize) -> Vector {
            Vector {name: "nothing".to_string(), components: vec![0.0;dimension]}
        }
    
        fn add(&self, v: Vector) -> Option<Vector> {
            let u = Vector::zero(self.components.len());

            Some(u)
        }


    }
}
