#[cfg(windows)]
fn main() {
    println!("cargo:rustc-link-arg=resources/resources.res");
}

#[cfg(unix)]
fn main() {}
