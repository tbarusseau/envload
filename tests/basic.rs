use envload::Envload;
// use envload_derive::Envload;

// #[derive(Envload)]
struct Env {
    secret_key: String,
    int_data: i32,
    optional_data: Option<usize>,
}

// The `Envload` impl derives to:
// ```rust
impl Envload for Env {
    fn load() -> Env {
        let secret_key: String = std::env::var("SECRET_KEY")
            .unwrap_or_else(|_| panic!("Environment variable not found: SECRET_KEY"))
            .parse()
            .unwrap_or_else(|_| panic!("Couldn't parse environment variable: SECRET_KEY"));
        let int_data: i32 = std::env::var("INT_DATA")
            .unwrap_or_else(|_| panic!("Environment variable not found: INT_DATA"))
            .parse()
            .unwrap_or_else(|_| panic!("Couldn't parse environment variable: INT_DATA"));
        let optional_data: Option<usize> = std::env::var("OPTIONAL_DATA").ok().map(|v| {
            v.parse()
                .unwrap_or_else(|_| panic!("Couldn't parse environment variable: OPTIONAL_DATA"))
        });

        // let optional_data = std::env::var("OPTIONAL_DATA").unwrap();

        Env {
            secret_key,
            int_data,
            optional_data,
        }
    }
}
// ```

#[test]
fn test_load_success() {
    std::env::set_var("SECRET_KEY", "hunter2");
    std::env::set_var("INT_DATA", "128");

    let env = <Env as Envload>::load();
    assert_eq!(env.secret_key, "hunter2");
    assert_eq!(env.int_data, 128);
    assert_eq!(env.optional_data, None);

    std::env::set_var("OPTIONAL_DATA", "37");

    let env = <Env as Envload>::load();
    assert_eq!(env.optional_data, Some(37));
}

#[test]
#[should_panic]
fn test_load_failure() {
    std::env::remove_var("SECRET_KEY");

    <Env as Envload>::load();

    // SECRET_KEY doesn't exist and isn't optional; panic!
}
