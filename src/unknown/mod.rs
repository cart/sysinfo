// Take a look at the license at the top of the repository in the LICENSE file.

pub mod component;
pub mod disk;
pub mod groups;
pub mod network;
pub mod users;

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
pub(crate) use self::disk::{DiskInner, DisksInner};
pub(crate) use self::groups::get_groups;
pub(crate) use self::network::{NetworkDataInner, NetworksInner};
pub(crate) use self::users::{get_users, UserInner};

#[doc = include_str!("../../md_doc/is_supported.md")]
pub const IS_SUPPORTED_SYSTEM: bool = false;
