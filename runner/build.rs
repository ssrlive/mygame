fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    copy_assets_to_root_dir();
}

fn copy_assets_to_root_dir() {
    use std::path::Path;

    let src = Path::new("assets");
    let dst = Path::new("../assets");

    if dst.exists() {
        std::fs::remove_dir_all(dst).unwrap();
    }

    let mut options = fs_extra::dir::CopyOptions::new();
    options.copy_inside = true;
    fs_extra::dir::copy(src, Path::new(".."), &options).unwrap();
}
