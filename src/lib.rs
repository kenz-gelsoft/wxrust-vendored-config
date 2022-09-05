#[cfg(windows)]
use wx_x86_64_pc_windows_msvc;

pub fn wx_config(args: &[&str]) -> Vec<String> {
    if cfg!(windows) {
        wx_x86_64_pc_windows_msvc::wx_config(args)
    } else {
        Vec::new()
    }
}
