#[macro_export]
macro_rules! asset_tree {
    ($name:ident { $($item_name:ident $(: $ty:ty )? $({ $($content:tt)* $(__ $content_flag:tt)? })?),* $(,)? } $(, $($rest:tt)*)?) => {
        #[allow(dead_code)]
        pub mod $name {
            use super::*;

            $(
                $(
                    $($content_flag)? #[allow(unused_imports)] use $item_name::*;
                )?
            )*

            $crate::asset_tree!($($item_name $(: $ty)? $({ $($content)* })?),*);

            $crate::paste! {
                pub struct [< $name:camel Folder>] {
                    $(
                        pub $item_name: $crate::asset_tree!(@resolve $item_name $(: $ty)? $({ $($content)* })?),
                    )*
                }

                impl $crate::StaticAssetFolder for [< $name:camel Folder>] {
                    fn name() -> &'static str {
                        stringify!($name)
                    }
                }

                impl $crate::Asset for [< $name:camel Folder>] {
                    fn bound() -> $crate::AssetBound {
                        $crate::AssetBound::Directory {
                            collect: vec![],
                            defined: vec![
                                $(
                                    $(
                                        $($content_flag)? <[< $item_name:camel Folder>] as $crate::StaticAssetFolder>::asset_tree(),
                                    )?
                                    $(
                                        $crate::AssetTreeNode {
                                            name: stringify!($item_name).to_string(),
                                            inner: <$ty>::bound(),
                                        },
                                    )?
                                )*
                            ],
                        }
                    }
                    fn load<L: $crate::loader::AssetLoader>(ctx: &L) -> $crate::Result<Self, L::Error> {
                        Ok(
                            Self {
                                $(
                                    $item_name: $crate::Asset::load(&ctx.subdir(stringify!($item_name)))?,
                                )*
                            }
                        )
                    }
                }
            }

        }

        $($crate::asset_tree!($($rest)*);)?
    };
    ($name:ident : $ty:ty $(, $($rest:tt)*)?) => {
        $($crate::asset_tree!($($rest)*);)?
    };
    () => {};

    (@resolve $name:ident : $ty:ty) => { $ty };
    (@resolve $name:ident { $($content:tt)* }) => {  $crate::paste! { [< $name:camel Folder >] }};
}

#[macro_export]
macro_rules! asset_files {
    ($name:ident : $ext:literal) => {
        impl $crate::Asset for $name {
            fn bound() -> $crate::AssetBound {
                $crate::AssetBound::File {
                    ty: $crate::AssetFileType {
                        extension: $ext.into(),
                    },
                }
            }
            fn load<L: $crate::loader::AssetLoader>(ctx: &L) -> $crate::Result<Self, L::Error> {
                let data = ctx.load_file($ext)?;
                let obj = data.try_into().map_err(|e| $crate::Error::deserialization(ctx.current_location().clone(), Box::new(e)))?;
                Ok(obj)
            }
        }
    };
    ($($name:ident : $ext:literal,)*) => { $( $crate::asset_files!($name : $ext); )* };
}
