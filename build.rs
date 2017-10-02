extern crate version_check;

#[cfg(feature = "use_union")]
fn check_version() {
    // Requires 1.20.0 for ManuallyDrop.
    if let Some((false, version)) = version_check::is_min_version("1.20.0") {
        panic!("arrayvec requires rustc 1.20.0, got {}", version);
    }
}

#[cfg(not(feature = "use_union"))]
fn check_version() { }

fn main() {
    check_version();
    println!("cargo:rerun-if-changed=build.rs");
}
