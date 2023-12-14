use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("Renderia test error.")]
    TestError,
}