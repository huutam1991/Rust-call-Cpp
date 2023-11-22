use std::env;
use std::fs;
use std::path::Path;

use git2::{Repository, SubmoduleUpdateOptions};
use std::process::Command;

fn main() {
    // Get the output directory for the build script
    let out_dir: String = env::var("OUT_DIR").unwrap();

    // Clone the Snappy GitHub repository to a subdirectory in the output directory
    let repo_url = "https://github.com/google/snappy.git";
    let repo_dir = Path::new(&out_dir).join("snappy");
    let repo = Repository::clone(repo_url, &repo_dir).unwrap();

    // Fetch the submodule in the Snappy repository
    let mut submodule_update_options = SubmoduleUpdateOptions::new();
    let submodules = repo.submodules().unwrap();
    for mut submodule in submodules {
        submodule.update(true, Some(&mut submodule_update_options)).unwrap();
    }

    // Create a folder for the build
    let build_dir = repo_dir.join("build");
    fs::create_dir(&build_dir).unwrap();

    // Run cmake and make in the build folder
    Command::new("cmake")
        .arg("../")
        .current_dir(&build_dir)
        .output()
        .unwrap();

    Command::new("make")
        .current_dir(&build_dir)
        .output()
        .unwrap();

    let src_file = Path::new(&out_dir).join("snappy/build/libsnappy.a");

    // Tell Cargo to link the library file
    println!("cargo:rustc-link-search=native={}", src_file.parent().unwrap().display());
    println!("cargo:rustc-link-lib=static=snappy");
    println!("cargo:rustc-link-lib=static=stdc++");
}
