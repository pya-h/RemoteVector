extern crate parser;
extern crate calculus;

use calculus::memory::Scope;
use parser::Analyzer;

fn sample_code(mut scope: Scope) {
    scope.define_vector("v".to_string(), vec![1.2, 2.6, 2.3, 4.5]);
    scope.define_vector("u".to_string(), vec![1.0, 2.0, 2.0, 4.0]);

    if let Some(v) = scope.get_vec(&"v".to_string()) {
        if let Some(u) = scope.get_vec(&"u".to_string()) {
            println!("{}", match v.add(u) {
                Some(z) => z.definition_string(),
                None => "Vector addition failed because of dimension mismatch".to_string(),
            });

            println!("{}", match v.sub(u) {
                Some(z) => z.definition_string(),
                None => "Vector subtract failed because of dimension mismatch".to_string(),
            });

            println!("{}", match v.plus_cv(u, 5.2) {
                Some(z) => z.definition_string(),
                None => "Vector v + cu failed because of dimension mismatch".to_string(),
            });

            println!("{}", match v.dot(u) {
                Some(z) => z.to_string(),
                None => "Vector inner product failed because of dimension mismatch".to_string(),
            });

            println!("v x u = {}", v.cross(u).to_string())
        }
    }
}
fn main() {
    let mut app = Analyzer::init();

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed reading statement!");
        println!("  => {}", app.analyze(&input, &String::from("main")));
    }
}
