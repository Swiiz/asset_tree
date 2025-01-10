use alloc::string::{String, ToString};

#[allow(unused_imports)]
use crate::{loader::AssetLoader, log, Asset, AssetBound, AssetTreeNode, Error, ErrorKind, Result};

#[cfg(not(feature = "no_std"))]
type HashMap<K, V> = std::collections::HashMap<K, V>;
#[cfg(feature = "no_std")]
type HashMap<K, V> = hashbrown::HashMap<K, V>;

#[cfg(not(feature = "no_std"))]
type Values<'a, K, V> = std::collections::hash_map::Values<'a, K, V>;
#[cfg(feature = "no_std")]
type Values<'a, K, V> = hashbrown::hash_map::Values<'a, K, V>;

pub struct Folder<T> {
    elems: HashMap<String, T>,
}

impl<T: Asset> Asset for Folder<T> {
    fn bound() -> AssetBound {
        AssetBound::Directory {
            collect: [T::bound()].into(),
            defined: [].into(),
        }
    }

    fn load<L: AssetLoader>(ctx: &L) -> Result<Self, L::Error> {
        Ok(Self {
            elems: ctx
                .iter_dir()?
                .filter_map(|node| match node {
                    Err(e) => Some(Err(e)),
                    Ok(node) => {
                        if node.inner.matches(&T::bound()) {
                            Some(T::load(&ctx.subdir(&node.name)).map(|res| (node.name, res)))
                        } else {
                            log!(
                                "Skipping non-matching asset: {}{}{}",
                                ctx.current_location(),
                                std::path::MAIN_SEPARATOR,
                                node.name
                            );
                            None
                        }
                    }
                })
                .collect::<Result<HashMap<_, _>, L::Error>>()?,
        })
    }
}

impl<T: Asset> Folder<T> {
    pub fn get_node(&self, name: &str) -> Option<AssetTreeNode<&T>> {
        self.get(name).map(|v| AssetTreeNode {
            name: name.to_string(),
            inner: v,
        })
    }

    pub fn get(&self, name: &str) -> Option<&T> {
        self.elems.get(name)
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = AssetTreeNode<&T>> + '_ {
        self.elems.iter().map(|(k, v)| AssetTreeNode {
            name: k.clone(),
            inner: v,
        })
    }
}

impl<'a, T: 'static> IntoIterator for &'a Folder<T> {
    type Item = &'a T;
    type IntoIter = Values<'a, String, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elems.values()
    }
}
