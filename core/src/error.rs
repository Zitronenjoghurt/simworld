pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error(transparent)]
    Image(#[from] image::ImageError),
    #[error("Invalid sprite sheet size")]
    InvalidSpriteSheetSize,
}
