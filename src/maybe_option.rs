#[doc(hidden)]
pub mod __private {
    use std::{marker::PhantomData, str::FromStr};

    pub struct MaybeOption<T>(PhantomData<T>);

    impl<T> MaybeOption<T> {
        pub fn new() -> Self {
            Self(PhantomData)
        }
    }

    impl<T: FromStr> MaybeOption<Option<T>> {
        pub fn generate(self, env_var: &str) -> Option<T>
        where
            <T as FromStr>::Err: std::fmt::Debug,
        {
            match std::env::var(env_var) {
                Ok(v) => Some(v.parse().expect("Parse error")),
                Err(e) => match e {
                    std::env::VarError::NotPresent => None,
                    std::env::VarError::NotUnicode(_) => panic!("Not unicode"),
                },
            }
        }
    }

    pub trait GenerateFallback {
        type Output;
        fn generate(self, env_var: &str) -> Self::Output;
    }

    impl<T: FromStr> GenerateFallback for MaybeOption<T> {
        type Output = T;
        fn generate(self, env_var: &str) -> Self::Output {
            match std::env::var(env_var) {
                Ok(v) => match v.parse() {
                    Ok(v) => v,
                    Err(_) => panic!("Parsing error"),
                },
                Err(e) => match e {
                    std::env::VarError::NotPresent => panic!("Not found"),
                    std::env::VarError::NotUnicode(_) => panic!("Not unicode"),
                },
            }
        }
    }
}

#[test]
fn test_api() {
    use __private::GenerateFallback;
    use __private::MaybeOption;

    std::env::set_var("SECRET_KEY", "0xcafebabe");
    std::env::set_var("OPTIONAL_DATA", "128");

    let secret_key: String = <MaybeOption<String>>::new().generate("SECRET_KEY");
    let optional_data: Option<usize> =
        <MaybeOption<Option<usize>>>::new().generate("OPTIONAL_DATA");

    assert_eq!(secret_key, String::from("0xcafebabe"));
    assert_eq!(optional_data, Some(128));

    std::env::remove_var("OPTIONAL_DATA");

    let optional_data: Option<usize> =
        <MaybeOption<Option<usize>>>::new().generate("OPTIONAL_DATA");

    assert_eq!(optional_data, None);
}
