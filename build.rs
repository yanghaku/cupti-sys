use std::{env, path::PathBuf};

static CUDA_VERSION: &'static str = "cuda-12.0";

macro_rules! cuda_panic {
    ( $name: expr ) => {
        panic!("{} cannot find! You can set the environment 'CUDA_LIBRARY_PATH' or 'CUDA_PATH' to specify it.", $name);
    };
}

fn find_cuda_lib(lib_name: &'static str) -> Vec<PathBuf> {
    let split_char;
    let dir_names;

    if cfg!(target_os = "windows") {
        split_char = ";";
        dir_names = vec!["", "x64", "lib\\x64"];
    } else {
        split_char = ":";
        dir_names = vec![
            "",
            "extras/CUPTI/lib64",
            "lib64",
            "stubs",
            "lib64/stubs",
            #[cfg(target_arch = "x86_64")]
            "targets/x86_64-linux",
            #[cfg(target_arch = "x86_64")]
            "targets/x86_64-linux/lib",
            #[cfg(target_arch = "aarch64")]
            "targets/aarch64-linux",
            #[cfg(target_arch = "aarch64")]
            "targets/aarch64-linux/lib",
        ];
    }

    let mut candidates: Vec<PathBuf> = env::var("CUDA_LIBRARY_PATH")
        .unwrap_or_default()
        .split(split_char)
        .map(|s| PathBuf::from(s))
        .collect();

    env::var("CUDA_PATH")
        .unwrap_or_default()
        .split(split_char)
        .for_each(|s| candidates.push(PathBuf::from(s)));

    let mut valid_paths = vec![];
    let mut target;

    #[cfg(not(target_os = "windows"))]
    {
        candidates.push(PathBuf::from("/opt/cuda"));
        candidates.push(PathBuf::from("/usr/local/cuda"));
        candidates.push(PathBuf::from("/usr/local/".to_string() + CUDA_VERSION));
    }

    for base in &candidates {
        if base.is_dir() {
            for dir_name in &dir_names {
                target = base.join(dir_name);
                if target.is_dir() && target.join(lib_name).is_file() {
                    valid_paths.push(target);
                }
            }
        }
    }
    valid_paths
}

fn find_cupti() -> Vec<PathBuf> {
    let lib_name;

    #[cfg(target_os = "windows")]
    {
        lib_name = "cupti.lib";
    }
    #[cfg(not(target_os = "windows"))]
    {
        #[cfg(feature = "static-link")]
        {
            lib_name = "libcupti_static.a";
        }
        #[cfg(not(feature = "static-link"))]
        {
            lib_name = "libcupti.so";
        }
    }

    let p = find_cuda_lib(lib_name);
    if p.is_empty() {
        cuda_panic!(lib_name);
    }
    p
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    for path in find_cupti() {
        println!("cargo:rustc-link-search=native={}", path.display());
    }
    if cfg!(not(target_os = "windows")) && cfg!(feature = "static-link") {
        println!("cargo:rustc-flags=-lcupti_static -lstdc++");
    } else {
        println!("cargo:rustc-flags=-lcupti");
    }
}
