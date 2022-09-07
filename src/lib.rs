use std::env;

#[cfg(feature = "vendored")]
pub fn wx_config(args: &[&str]) -> Vec<String> {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os.eq("windows") {
        let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();
        if target_env.eq("msvc") {
            wx_x86_64_pc_windows_msvc::wx_config(args)
        } else {
            wx_x86_64_pc_windows_gnu::wx_config(args)
        }
    } else if target_os.eq("macos") {
        wx_universal_apple_darwin::wx_config(args)
    } else {
        Vec::new()
    }
}

#[cfg(all(not(feature = "vendored"), not(windows)))]
pub fn wx_config(args: &[&str]) -> Vec<String> {
    let output = Command::new("wx-config")
        .args(args)
        .output()
        .expect("failed execute wx-config command.");
    let flags: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .to_string()
        .split_whitespace()
        .map(ToOwned::to_owned)
        .collect();
    let mut converted = Vec::new();
    let mut next_is_framework_name = false;
    for flag in flags.iter() {
        if next_is_framework_name {
            converted.push(format!("-lframework={}", flag));
            next_is_framework_name = false;
        } else if flag == "-framework" {
            next_is_framework_name = true;
        } else if flag.starts_with("-pthread") {
            // ignore
        } else {
            converted.push(flag.to_string());
        }
    }
    converted
}

#[cfg(all(not(feature = "vendored"), windows))]
pub fn wx_config(args: &[&str]) -> Vec<String> {
    let wxwin = env::var("wxwin")
        .expect("Set 'wxwin' environment variable to point the wxMSW binaries dir.");
    // TODO: support linking with the wx debug DLL
    let is_debug = false; //env::var("PROFILE").unwrap() == "debug";
    let d_or_not = if is_debug { "d" } else { "" };
    if args.contains(&"--cflags") {
        let mut cflags = vec![
            format!("-I{}\\include", wxwin),
            format!("-I{}\\lib\\vc14x_x64_dll\\mswu{}", wxwin, d_or_not),
            "-DWXUSINGDLL".to_string(),
        ];
        if is_debug {
            cflags.push("-D_DEBUG".to_string());
        } else {
            cflags.push("-D__NO_VC_CRTDBG__".to_string());
            cflags.push("-DwxDEBUG_LEVEL=0".to_string());
            cflags.push("-DNDEBUG".to_string());
        }
        let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();
        if target_env.eq("msvc") {
            cflags.push("/EHsc".to_string());
        }
        cflags
    } else {
        let libs = vec![
            format!("-L{}\\lib\\vc14x_x64_dll", wxwin),
            format!("-lwxbase32u{}", d_or_not),
            format!("-lwxmsw32u{}_core", d_or_not),
        ];
        libs
    }
}
