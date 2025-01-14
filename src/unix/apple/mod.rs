// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(any(target_os = "ios", feature = "apple-sandbox"))]
pub(crate) mod app_store;

pub mod component;
pub mod disk;
mod ffi;
pub mod groups;
pub mod network;
pub mod users;
mod utils;

cfg_if::cfg_if! {
    if #[cfg(target_os = "macos")] {
        pub(crate) mod macos;
        pub(crate) use self::macos as inner;
    } else if #[cfg(target_os = "ios")] {
        pub(crate) mod ios;
        pub(crate) use self::ios as inner;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "system")] {
        pub mod cpu;
        pub mod process;
        pub mod system;

        pub(crate) use self::cpu::CpuInner;
        pub(crate) use self::process::ProcessInner;
        pub(crate) use self::system::SystemInner;
        pub use self::system::{MINIMUM_CPU_UPDATE_INTERVAL, SUPPORTED_SIGNALS};
    }
}

pub(crate) use self::component::{ComponentInner, ComponentsInner};
pub(crate) use self::disk::DiskInner;
pub(crate) use self::network::{NetworkDataInner, NetworksInner};
pub(crate) use crate::unix::groups::get_groups;
pub(crate) use crate::unix::users::{get_users, UserInner};
pub(crate) use crate::unix::DisksInner;

#[doc = include_str!("../../../md_doc/is_supported.md")]
pub const IS_SUPPORTED_SYSTEM: bool = true;
