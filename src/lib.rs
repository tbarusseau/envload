pub mod maybe_option;

pub trait Envload {
    fn load() -> Self;
}
