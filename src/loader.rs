use crate::{log, AssetProperties, AssetTreeNode, Error, Result};
use std::path::{Path, PathBuf};

pub trait AssetLoader: Sized {
    type Error: std::error::Error;
    fn new(root: impl Into<PathBuf>) -> Result<Self, Self::Error>;
    fn current_path(&self) -> &Path;
    fn subdir(&self, name: &str) -> Self;
    fn load_file(&self, ext: &str) -> Result<Vec<u8>, Self::Error>;
    fn iter_dir(
        &self,
    ) -> Result<
        impl Iterator<Item = Result<AssetTreeNode<AssetProperties>, Self::Error>> + '_,
        Self::Error,
    >;
}

pub struct StdOsLoader {
    parent_path: PathBuf,
}

impl AssetLoader for StdOsLoader {
    type Error = std::io::Error;

    fn new(root: impl Into<PathBuf>) -> Result<Self, Self::Error> {
        let path = root.into();
        Ok(Self {
            parent_path: std::fs::canonicalize(&path)
                .map_err(|e| Error::<Self::Error>::loader(path, e))?,
        })
    }

    fn current_path(&self) -> &Path {
        &self.parent_path
    }

    fn subdir(&self, name: &str) -> StdOsLoader {
        StdOsLoader {
            parent_path: self.parent_path.join(name),
        }
    }

    fn load_file(&self, ext: &str) -> Result<Vec<u8>, Self::Error> {
        std::fs::read(self.parent_path.with_extension(ext))
            .map_err(|e| Error::loader(self.parent_path.clone(), e))
    }

    fn iter_dir(
        &self,
    ) -> Result<
        impl Iterator<Item = Result<AssetTreeNode<AssetProperties>, Self::Error>> + '_,
        Self::Error,
    > {
        let dir = std::fs::read_dir(&self.parent_path)
            .map_err(|e| Error::loader(self.parent_path.clone(), e))?;

        Ok(dir
            .into_iter()
            .map(|entry| {
                Ok(entry
                    .map_err(|e| Error::loader(self.parent_path.clone(), e))?
                    .path())
            })
            .filter_map(|e: Result<_, Self::Error>| {
                if let Ok(path) = e {
                    Some(path)
                } else {
                    log!("Skipping dir entry: {}", e.unwrap_err());
                    None
                }
            })
            .map(|path: PathBuf| {
                Ok(if path.is_dir() {
                    let name = path.file_name().unwrap().to_string_lossy().to_string();
                    AssetTreeNode {
                        inner: AssetProperties::Directory {
                            children: self
                                .subdir(&name)
                                .iter_dir()?
                                .collect::<Result<_, Self::Error>>()?,
                        },
                        name,
                    }
                } else {
                    let name = path.file_stem().unwrap().to_string_lossy().to_string();
                    AssetTreeNode {
                        inner: AssetProperties::File {
                            ext: path.extension().unwrap().to_string_lossy().to_string(),
                        },
                        name,
                    }
                })
            }))
    }
}
