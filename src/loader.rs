use alloc::{string::String, vec::Vec};

#[allow(unused_imports)]
use crate::{log, AssetProperties, AssetTreeNode, Error, Result};

#[cfg(not(feature = "no_std"))]
use std::path::{Path, PathBuf};

pub trait AssetLoader: Sized {
    type Error: core::error::Error;
    fn new(root: String) -> Result<Self, Self::Error>;
    fn current_location(&self) -> String;
    fn subdir(&self, name: &str) -> Self;
    fn check_file(&self, ext: &str) -> Result<bool, Self::Error>;
    fn load_file(&self, ext: &str) -> Result<Vec<u8>, Self::Error>;
    fn iter_dir(
        &self,
    ) -> Result<
        impl Iterator<Item = Result<AssetTreeNode<AssetProperties>, Self::Error>> + '_,
        Self::Error,
    >;
}

#[cfg(not(feature = "no_std"))]
pub struct StdOsLoader {
    parent_path: PathBuf,
}

#[cfg(not(feature = "no_std"))]
impl StdOsLoader {
    pub fn current_path(&self) -> &Path {
        &self.parent_path
    }
}

#[cfg(not(feature = "no_std"))]
impl AssetLoader for StdOsLoader {
    type Error = std::io::Error;

    fn new(root: String) -> Result<Self, Self::Error> {
        let path = root.into();
        Ok(Self {
            parent_path: std::fs::canonicalize(&path)
                .map_err(|e| Error::<Self::Error>::loader(path, e))?,
        })
    }

    fn current_location(&self) -> String {
        self.parent_path.display().to_string()
    }

    fn subdir(&self, name: &str) -> StdOsLoader {
        StdOsLoader {
            parent_path: self.parent_path.join(name),
        }
    }

    fn check_file(&self, ext: &str) -> Result<bool, Self::Error> {
        let path = self.parent_path.with_extension(ext);
        Ok(path
            .try_exists()
            .map_err(|e| Error::loader(path.display().to_string(), e))?
            && path.is_file())
    }

    fn load_file(&self, ext: &str) -> Result<Vec<u8>, Self::Error> {
        std::fs::read(self.parent_path.with_extension(ext))
            .map_err(|e| Error::loader(self.parent_path.display().to_string(), e))
    }

    fn iter_dir(
        &self,
    ) -> Result<
        impl Iterator<Item = Result<AssetTreeNode<AssetProperties>, Self::Error>> + '_,
        Self::Error,
    > {
        let dir = std::fs::read_dir(&self.parent_path)
            .map_err(|e| Error::loader(self.parent_path.display().to_string(), e))?;

        Ok(dir
            .into_iter()
            .map(|entry| {
                Ok(entry
                    .map_err(|e| Error::loader(self.parent_path.display().to_string(), e))?
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
