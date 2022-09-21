use std::env;

fn main() {
    let pwd = get_present_working_dir();
    println!("cargo:rustc-link-search=native={}{}", pwd, "/../networking/platform/mac/.build/arm64-apple-macosx/debug");
}

fn get_present_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}