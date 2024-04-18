use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use relative_path::RelativePath;

use crate::{Compose, ComposeBuilderError};

#[derive(Default)]
pub struct ComposeBuilder {
    path: Option<String>,
}

impl ComposeBuilder {
    /// Create a new ComposeBuilder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the path to the docker-compose file.
    /// The path can be either absolute or relative.
    pub fn path(mut self, path: impl ToString) -> Self {
        self.path = Some(path.to_string());
        self
    }

    /// Build the Compose object.
    ///
    /// # Errors
    ///
    /// Returns a [ComposeBuilderError] if the path is missing or the file is not found.
    pub fn build(self) -> Result<Compose, ComposeBuilderError> {
        let path = match self.path {
            Some(path) => path,
            None => return Err(ComposeBuilderError::MissingField("path".to_string())),
        };

        let path = match Path::new(&path).is_absolute() {
            true => PathBuf::from(path),
            false => {
                let base = current_dir()?;
                RelativePath::new(&path).to_logical_path(base)
            }
        };

        if path.exists() {
            Ok(Compose {
                path: path.to_string_lossy().to_string(),
            })
        } else {
            Err(ComposeBuilderError::FileNotFound(
                path.to_string_lossy().to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose_builder() {
        let compose = Compose::builder().abs_path("docker-compose.yml").build();

        matches!(compose, Ok(_));
    }

    #[test]
    fn test_compose_builder_missing_field() {
        let compose = Compose::builder().build();
        assert!(compose.is_err());
        matches!(compose, Err(ComposeBuilderError::MissingField(_)));
    }

    #[test]
    fn test_compose_builder_file_not_found() {
        let compose = Compose::builder().abs_path("non-existent-file.yml").build();
        assert!(compose.is_err());
        matches!(compose, Err(ComposeBuilderError::FileNotFound));
    }
}
