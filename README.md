<h1 align=center>📁 Asset Tree 🌲</h1>

## Rust library to easily load/check/access assets using trees and macros.

Allows for static and dynamic file hierarchies loading, integrity checking, reloading and more with minimal runtime logic using the rust type system.</br>
Also support `#[no_std]` environments and provide the ability to define custom asset tree loaders (for virtual file systems/embedded platforms...).

## Features

- Easy to use macro to statically define asset trees
- `StdOsLoader` for loading assets from the file system + ability to define custom asset loaders
- Support for `#[no_std]` environment. (with `alloc` dependency)
- Ability to load/reload any part of the asset tree when needed
- Ability to check the integrity of the asset tree when needed
- Runtime asset tree data/bound representation
- Error handling and unused assets logging for easier debugging
- Custom file types with loader independant logic
- Minimal runtime logic

## Usage

- Add the dependency to your `Cargo.toml` file:
```toml
[dependencies]
asset_tree = { git = "https://github.com/Swiiz/asset_tree" }
```

- Define your file types (This will implement the Asset trait for each file type)</br>
⚠ For this to work you need to implement `From<Vec<u8>>` or `TryFrom<Vec<u8>>` for each file type.
```rust
asset_files! {
  Texture : "png",
  Blueprint : "bp",
}
```

- Define your asset tree (This generates a AssetsFolder, TexturesFolder and BlueprintsFolder types with their respective fields)</br>
ℹ The `builtin::Folder` type will automatically collect all assets in the subfolder matching the generic asset.
```rust
asset_tree! {
    assets {
        textures {
            house : Texture,
            garden : Texture,
        },
        blueprints : builtin::Folder<Blueprint>,
    },
}
```

- You can now check the integrity of the asset tree / load it using the root struct (AssetsFolder).
```rust
// Checks the integrity of the asset tree
let missing = check_integrity(&assets::AssetsFolder::asset_tree(), ctx).unwrap();
if !missing.is_empty() {
    panic!("Missing assets: {:?}", missing);
}

// Loads the asset tree
let assets = AssetsFolder::load().unwrap();
```

- You can also reload the asset tree at any time
```rust
assets.reload().unwrap();
```

- Lastly you can access the assets in the tree safely and easily
```rust
let house_texture = assets.textures.house;
let blueprint = assets.blueprints.iter_nodes(); //or <&BlueprintsFolder as IntoIterator>::into_iter() to only get values
```

## Example(s)

- ### [Full example](https://github.com/Swiiz/asset_tree/tree/master/examples/full.rs)
  **The main features showcased in a single example.**

  Run the example:
  ```
  cargo run --example full
  ```
  See the generated code documentation:
  ```
  cargo doc --example full --no-deps --open 
  ```
****************************

> [!NOTE]
> This is a work in progress. Documentation needs to be written/improved.
> 
> Feel free to open an issue or contribute! Any help is appreciated 😁
