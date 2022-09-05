#[cfg(target_env = "msvc")]
pub fn wx_config(args: &[&str]) -> Vec<String> {
    wx_x86_64_pc_windows_msvc::wx_config(args)
}
