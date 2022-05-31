/*!
A derive macro for automatically filling a struct based on the current environment.

# Example

```rust
# use envload::{Envload, LoadEnv};
# use std::env;
# #[derive(PartialEq, Eq, Debug)]
#[derive(Envload)]
struct Env {
    secret_key: String,
    int_data: i32, // Any type that implements `FromStr` can be used
    optional_data: Option<usize>,
}

// Setup environment variables...
env::set_var("SECRET_KEY", "hunter2");
env::set_var("INT_DATA", "128");
env::remove_var("OPTIONAL_DATA");

// ... Struct can now be loaded from current environment.
// Any missing non-`Option` field results in a panic.
let env = <Env as LoadEnv>::load_env();
assert_eq!(env, Env {
    secret_key: String::from("hunter2"),
    int_data: 128,
    optional_data: None
});

// Add data for `optional_data` field...
env::set_var("OPTIONAL_DATA", "37");

// ... And it's now available!
let env = <Env as LoadEnv>::load_env();
assert_eq!(env.optional_data, Some(37));
```

# Motivation

In almost every codebase where I rely on environment variable, I end up writing a `Env` struct which fill its fields
based on what's currently in the environment.

Usually, I have to define a list of mandatory variables, and then I have to convert the data myself.

I thought that given how powerful Rust's macros are, it would be a good fit for a first proc macro!

Combined with [`dotenv`](https://github.com/dotenv-rs/dotenv), this makes for relatively painless environment variables management!
*/

extern crate envload_derive;
pub use envload_derive::Envload;

pub mod maybe_option;

/// Main trait, exposing the [`LoadEnv::load_env`] method
pub trait LoadEnv {
    /// Loads `Self` with whatever variables available in the current environment.
    fn load_env() -> Self;
}
