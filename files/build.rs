use std::env;
use swift_rs::build;

fn main() {
    let target = env::var("CARGO_CFG_TARGET_OS").unwrap();

    if target == "macos" {
        build::link_swift();
        build::link_swift_package("platformMac", &format!("{}/platform/mac/", env!("CARGO_MANIFEST_DIR")));
    }
}