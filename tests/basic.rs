use envload::Envload;
use envload_derive::Envload;

#[derive(Envload)]
struct Env {
    secret_key: String,
    int_data: i32,
    // optional_data: Option<usize>,
}

#[test]
fn load_success() {
    std::env::set_var("SECRET_KEY", "hunter2");
    std::env::set_var("INT_DATA", "128");

    let env = <Env as Envload>::load();
    assert_eq!(env.secret_key, "hunter2");
    assert_eq!(env.int_data, 128);
    // assert_eq!(env.optional_data, None);

    // std::env::set_var("OPTIONAL_DATA", "37");

    // let env = <Env as Envload>::load();
    // assert_eq!(env.optional_data, Some(37));
}

#[test]
#[should_panic]
fn load_failure() {
    std::env::remove_var("SECRET_KEY");

    <Env as Envload>::load();

    // SECRET_KEY doesn't exist and isn't optional; panic!
}
