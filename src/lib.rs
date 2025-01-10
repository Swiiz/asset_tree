pub mod builtin;
mod error;
pub mod loader;
mod macros;

#[doc(hidden)]
pub use paste::paste;

pub use error::Error;
pub type Result<T, E> = std::result::Result<T, Error<E>>;

#[cfg(not(feature = "no_log"))]
#[doc(hidden)]
pub static DEBUG_LOG: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new({
    #[cfg(debug_assertions)]
    let b = true;
    #[cfg(any(not(debug_assertions)))]
    let b = false;
    b
});

#[cfg(not(feature = "no_log"))]
pub fn set_debug_log(b: bool) {
    DEBUG_LOG.store(b, std::sync::atomic::Ordering::Relaxed);
}

macro_rules! log {
    ($($arg:tt)*) => {
        #[cfg(not(feature = "no_log"))]
        if $crate::DEBUG_LOG.load(std::sync::atomic::Ordering::Relaxed) {
            println!("DEBUG (asset_tree): {}", format!($($arg)*));
        }
    };
}
pub(crate) use log;

#[derive(Debug, Clone, PartialEq)]
pub struct AssetFileType {
    pub extension: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssetTreeNode<T = AssetBound> {
    pub name: String,
    pub inner: T,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssetBound {
    File {
        ty: AssetFileType,
    },
    Directory {
        collect: Vec<AssetBound>,
        defined: Vec<AssetTreeNode>,
    },
}

pub trait StaticAssetFolder: Asset {
    fn name() -> &'static str;
    fn asset_tree() -> AssetTreeNode {
        AssetTreeNode {
            name: Self::name().to_string(),
            inner: Self::bound(),
        }
    }
}
pub trait Asset: Sized {
    fn bound() -> AssetBound;
    fn load<L: loader::AssetLoader>(ctx: &L) -> Result<Self, L::Error>;

    fn reload<L: loader::AssetLoader>(&mut self, ctx: &L) -> Result<(), L::Error> {
        *self = Self::load(ctx)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssetProperties {
    File {
        ext: String,
    },
    Directory {
        children: Vec<AssetTreeNode<AssetProperties>>,
    },
}

impl AssetProperties {
    pub fn matches(&self, bound: &AssetBound) -> bool {
        match (self, bound) {
            (AssetProperties::File { ext }, AssetBound::File { ty }) => *ext == ty.extension,
            (
                AssetProperties::Directory { children },
                AssetBound::Directory { collect, defined },
            ) => children.iter().all(|node| {
                collect.iter().any(|b| node.inner.matches(b))
                    || defined
                        .iter()
                        .any(|b| node.inner.matches(&b.inner) && b.name == node.name)
            }),
            _ => false,
        }
    }
}
