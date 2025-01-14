// Take a look at the license at the top of the repository in the LICENSE file.

pub mod disk;
pub mod ffi;
pub(crate) mod utils;

#[cfg(all(feature = "system", not(feature = "apple-sandbox")))]
pub(crate) mod cpu;

#[cfg(all(feature = "system", not(feature = "apple-sandbox")))]
pub mod system;

#[cfg(not(feature = "apple-sandbox"))]
pub mod component;

#[cfg(all(feature = "system", not(feature = "apple-sandbox")))]
pub mod process;

#[cfg(feature = "apple-sandbox")]
pub use crate::sys::app_store::component;

#[cfg(all(feature = "system", feature = "apple-sandbox"))]
pub use crate::sys::app_store::process;
