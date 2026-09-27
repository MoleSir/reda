use std::path::Path;
use std::process::Command;

fn main() {
    let def_source_dir = Path::new("third_party/si2-def/def");
    let cdef_source_dir = Path::new("third_party/si2-def/cdef");

    let def_y = def_source_dir.join("def.y");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    let def_tab_cpp = out_dir.join("def.tab.cpp");

    // run bison
    let status = Command::new("bison")
        .args(&["-v", "-p", "defyy", "-d", def_y.to_str().unwrap(), "-o", def_tab_cpp.to_str().unwrap()])
        .status()
        .expect("Failed to run bison");
    if !status.success() {
        panic!("bison returned an error");
    }

    // collect cpp files
    let mut cpp_files = vec![def_tab_cpp.clone()];
    for dir in &[def_source_dir, cdef_source_dir] {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|x| x.to_str()) == Some("cpp")
                && path.file_name() != Some(std::ffi::OsStr::new("def.tab.cpp"))
            {
                cpp_files.push(path);
            }
        }
    }

    // compile
    let mut build = cc::Build::new();
    build.cpp(true);
    build.flag_if_supported("-w");
    build.flag("-std=c++17");
    build.include(def_source_dir);
    build.include(out_dir); // include bison output
    for file in &cpp_files {
        build.file(file);
    }
    build.compile("si2_def");

    println!("cargo:rustc-link-lib=static=si2_def");
}
