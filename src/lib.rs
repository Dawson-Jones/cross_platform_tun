mod configuration;
pub use configuration::Configuration;

mod address;
mod error;
mod interface;

mod platform;
pub use platform::tun;

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
mod r#async {
    pub mod codec;
    pub mod tun;
}
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
pub use r#async::{codec::TunPacket, codec::TunPacketCodec, tun::AsyncTun};
