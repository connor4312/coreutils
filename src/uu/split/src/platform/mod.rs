#[cfg(any(unix, target_os = "wasi"))]
pub use self::unix::instantiate_current_writer;
#[cfg(any(unix, target_os = "wasi"))]
pub use self::unix::paths_refer_to_same_file;

#[cfg(windows)]
pub use self::windows::instantiate_current_writer;
#[cfg(windows)]
pub use self::windows::paths_refer_to_same_file;

#[cfg(any(unix, target_os = "wasi"))]
mod unix;

#[cfg(windows)]
mod windows;
