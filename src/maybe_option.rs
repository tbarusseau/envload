//! Inherent specialization struct

// Thanks a lot to Sabrina Jewson for the code: https://sabrinajewson.org/

#[doc(hidden)]
pub mod __private {
    use std::{marker::PhantomData, str::FromStr};

    // use crate::errors::EnvloadError;

    pub struct MaybeOption<T>(PhantomData<T>);

    impl<T> MaybeOption<T> {
        pub fn new() -> Self {
            Self(PhantomData)
        }
    }

    // Result<T, E> implementation
    // impl<T: FromStr> MaybeOption<Result<T, EnvloadError>> {
    //     pub fn generate(self, env_var: &str) -> Result<T, EnvloadError> {
    //         match std::env::var(env_var) {
    //             Ok(v) => v.parse().map_err(|_| EnvloadError::ParseError),
    //             Err(e) => match e {
    //                 std::env::VarError::NotPresent => Err(EnvloadError::EnvVarNotFound),
    //                 std::env::VarError::NotUnicode(_) => Err(EnvloadError::InvalidUnicodeData),
    //             },
    //         }
    //     }
    // }

    // Option<T> implementation
    impl<T: FromStr> MaybeOption<Option<T>> {
        pub fn generate(self, env_var: &str) -> Option<T> {
            match std::env::var(env_var) {
                Ok(v) => v.parse().ok(),
                Err(e) => match e {
                    std::env::VarError::NotPresent => None,
                    std::env::VarError::NotUnicode(_) => None,
                },
            }
        }
    }

    pub trait GenerateFallback {
        type Output;
        fn generate(self, env_var: &str) -> Self::Output;
    }

    // Fallback implementation: returns parsed value or panics.
    impl<T: FromStr> GenerateFallback for MaybeOption<T> {
        type Output = T;

        fn generate(self, env_var: &str) -> Self::Output {
            match std::env::var(env_var) {
                Ok(v) => match v.parse() {
                    Ok(v) => v,
                    Err(_) => panic!("Error while parsing environment variable: {} (does the value match the struct type?)", env_var),
                },
                Err(e) => match e {
                    std::env::VarError::NotPresent => {
                        panic!("Environment variable not found: {}", env_var)
                    }
                    std::env::VarError::NotUnicode(_) => panic!(
                        "Environment variable contains invalid unicode data: {}",
                        env_var
                    ),
                },
            }
        }
    }
}

#[test]
fn test_api() {
    use __private::GenerateFallback;
    use __private::MaybeOption;

    // use crate::errors::EnvloadError;

    std::env::set_var("SECRET_KEY", "hunter2");
    // let as_result = <MaybeOption<Result<String, EnvloadError>>>::new().generate("SECRET_KEY");
    let as_option = <MaybeOption<Option<String>>>::new().generate("SECRET_KEY");
    let as_value = <MaybeOption<String>>::new().generate("SECRET_KEY");

    // assert_eq!(as_result.unwrap(), String::from("hunter2"));
    assert_eq!(as_option.unwrap(), String::from("hunter2"));
    assert_eq!(as_value, String::from("hunter2"));

    // let as_optional_result =
    //     <MaybeOption<Result<Option<String>, EnvloadError>>>::new().generate("SECRET_KEY");

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
