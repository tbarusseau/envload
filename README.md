# envload

A derive macro for automatically filling a struct based on the current environment.

## Example

```rust
#[derive(Envload)]
struct Env {
    secret_key: String,
    int_data: i32,
    optional_data: Option<usize>,
}

// Setup environment variables...
env::set_var("SECRET_KEY", "hunter2");
env::set_var("INT_DATA", "128");
env::remove_var("OPTIONAL_DATA");

// ... Struct can now be access
let env = <Env as Envload>::load();
assert_eq!(env, Env { secret_key: "hunter2", int_data: 128, optional_data: None });

// Add data for `optional_data` field...
env::set_var("OPTIONAL_DATA", "37");

let env = <Env as Envload>::load();
assert_eq!(env.optional_data, Some(37));
```

## Motivation

In almost every codebase where I rely on environment variable, I end up writing a `Env` struct which fill its fields
based on what's currently in the environment.

Usually, I have to define a list of mandatory variables, and then I have to convert the data myself.

I thought that given how powerful Rust's macros are, it would be a good fit for a first proc macro!