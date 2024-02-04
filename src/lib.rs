mod configuration;
pub use configuration::Configuration;

mod address;
mod error;
mod interface;

mod platform;
pub use platform::create;


#[cfg(all(
    feature = "async",
    any(
        target_os = "windows",
        target_os = "linux",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )
))]
mod r#async;
#[cfg(all(
    feature = "async",
    any(
        target_os = "windows",
        target_os = "linux",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )
))]
pub use r#async::create_as_async;
pub use r#async::TunPacket;