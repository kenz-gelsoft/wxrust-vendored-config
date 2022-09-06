#[cfg(target_env = "msvc")]
pub fn wx_config(args: &[&str]) -> Vec<String> {
    wx_x86_64_pc_windows_msvc::wx_config(args)
}

#[cfg(all(windows, target_env = "gnu"))]
pub fn wx_config(args: &[&str]) -> Vec<String> {
    wx_x86_64_pc_windows_gnu::wx_config(args)
}

#[cfg(target_os = "macos")]
pub fn wx_config(args: &[&str]) -> Vec<String> {
    wx_universal_apple_darwin::wx_config(args)
}
