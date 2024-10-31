use vecalc::calculus;

fn main() {
    let mut app = calculus::Analyzer::init();
    let main_scope = app.get().get("main".to_string());
    main_scope.define_vector("v1".to_string(), vec![1.2, 2.6, 2.3, 4.5]);

    let new_v = main_scope.get(&"v1".to_string());
    match new_v {
        Some(&ref v) => println!("{}", v.to_string()),
        None => println!("v1 not found."),
    }
}
