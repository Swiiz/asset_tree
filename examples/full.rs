#[allow(unused_imports)]
use {
    asset_tree::{
        asset_tree, builtin, check_integrity, loader::AssetLoader, Asset, StaticAssetFolder,
    },
    files::*,
};

#[cfg(not(feature = "no_std"))]
use asset_tree::loader::StdOsLoader;

#[cfg(not(feature = "no_std"))]
fn main() {
    let ctx = StdOsLoader::new("examples/assets".into()).unwrap();
    let mut assets = assets::AssetsFolder::load(&ctx).unwrap();

    loop {
        assets.reload(&ctx).unwrap();

        println!("Shader source: {}", assets.shaders.my_shader.source_code);
        println!(
            "assets.textures asset tree: {:?}",
            assets::textures::TexturesFolder::asset_tree()
        );
        println!(
            "blueprints: {:?}",
            assets
                .blueprints
                .iter_nodes()
                .map(|bp| bp.name)
                .collect::<Vec<_>>()
        );
        println!(
            "plugins: {:?}",
            assets
                .plugins
                .iter_nodes()
                .map(|p| p.name)
                .collect::<Vec<_>>()
        );

        println!(
            "Press enter to reload assets. Debug logs can be turned off/on using the no_log feature or the `set_debug_log` function."
        );
        let _ = std::io::stdin().read_line(&mut String::new());

        let missing = check_integrity(&assets::AssetsFolder::asset_tree(), &ctx).unwrap();
        if !missing.is_empty() {
            panic!("Missing assets: {:?}", missing);
        }
    }
}

asset_tree! {
    assets {
        textures {
            my_texture : Texture,
        },
        shaders {
            my_shader : Shader,
        },
        blueprints : builtin::Folder<Blueprint>,
        plugins: builtin::Folder<plugin::PluginFolder>,
    },

    plugin {
        manifest: Json
    }
}

pub mod files {
    asset_tree::asset_files! {
        Texture : "png",
        Shader : "glsl",
        Blueprint : "bp",
        Json : "json",
    }

    pub struct Texture;
    pub struct Shader {
        pub source_code: String,
    }
    pub struct Blueprint;
    pub struct Json;

    impl From<Vec<u8>> for Texture {
        fn from(_: Vec<u8>) -> Self {
            Texture
        }
    }

    impl TryFrom<Vec<u8>> for Shader {
        type Error = std::string::FromUtf8Error;

        fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
            Ok(Shader {
                source_code: String::from_utf8(value)?,
            })
        }
    }

    impl From<Vec<u8>> for Blueprint {
        fn from(_: Vec<u8>) -> Self {
            Blueprint
        }
    }

    impl From<Vec<u8>> for Json {
        fn from(_: Vec<u8>) -> Self {
            Json
        }
    }
}

#[cfg(feature = "no_std")]
fn main() {
    panic!("The example doesn't support std, you need to write your own loader using the AssetLoader trait");
}
