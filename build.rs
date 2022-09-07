use std::env;

fn dep_links() -> String {
    let target = env::var("TARGET").unwrap().replace('-', "_").to_uppercase();
    if target.contains("APPLE") {
        "UNIVERSAL_APPLE_DARWIN".to_owned()
    } else {
        target
    }
}

fn main() {
    // transfer flags
    let flags = env::var(format!("DEP_WX_{}_CFLAGS", dep_links())).unwrap();
    println!("cargo:cflags={}", flags);
}
