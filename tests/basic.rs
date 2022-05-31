#[cfg(test)]
mod tests {
    use envload::{Envload, LoadEnv};

    #[derive(Envload)]
    struct Env {
        secret_key: String,
        int_data: i32,
        optional_data: Option<usize>,
    }

    #[test]
    fn load_success() {
        // Make sure our environment is properly setup
        std::env::set_var("SECRET_KEY", "hunter2");
        std::env::set_var("INT_DATA", "128");
        std::env::remove_var("OPTIONAL_DATA");

        let env = <Env as LoadEnv>::load_env();
        assert_eq!(env.secret_key, "hunter2");
        assert_eq!(env.int_data, 128);
        assert_eq!(env.optional_data, None);

        // Add the optional data to our current env
        std::env::set_var("OPTIONAL_DATA", "37");

        let env = <Env as LoadEnv>::load_env();
        assert_eq!(env.optional_data, Some(37));
    }

    #[test]
    #[should_panic]
    fn load_failure() {
        use envload::LoadEnv;

        std::env::remove_var("SECRET_KEY");

        <Env as LoadEnv>::load_env();

        // SECRET_KEY doesn't exist and isn't optional; panic!
    }
}
