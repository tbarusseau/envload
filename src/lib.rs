pub enum EnvloadError {}

pub trait Envload {
    fn load() -> Self;
    // fn try_load() -> Result<Self, EnvloadError>
    // where
    //     Self: Sized;
}
