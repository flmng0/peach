use thiserror::Error;

#[derive(Error, Debug)]
pub enum SketchError {
    #[error("error creating window")]
    WindowCreation(#[from] winit::error::OsError),
}
