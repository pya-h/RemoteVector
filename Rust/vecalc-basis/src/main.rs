use vecalc::calculus::Vector;

#[macro_export] macro_rules! V {
        ($($x:expr), *) => {
            {
                let mut v: Vector = Vector::new(&"nothing".to_string(), Vec::new());
                $(
                    v.add_component($x);
                )*
                v
            }
        };
}

fn main() {
    let v = V!(1.2, 2.6, 2.3, 4.5);
    let u = V!(1.0, 2.0, 2.0, 4.0);
    
    println!(
        "{}",
        match v.add(&u) {
            Some(z) => z.definition_string(),
            None => "Vector addition failed because of dimension mismatch".to_string(),
        }
    );

    println!(
        "{}",
        match v.sub(&u) {
            Some(z) => z.definition_string(),
            None => "Vector subtract failed because of dimension mismatch".to_string(),
        }
    );

    println!(
        "{}",
        match v.plus_cv(&u, 5.2) {
            Some(z) => z.definition_string(),
            None => "Vector v + cu failed because of dimension mismatch".to_string(),
        }
    );

    println!(
        "{}",
        match v.dot(&u) {
            Some(z) => z.to_string(),
            None => "Vector inner product failed because of dimension mismatch".to_string(),
        }
    );

    println!(
        "v x u = {}", v.cross(&u).to_string()
    )
}
