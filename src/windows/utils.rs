// Take a look at the license at the top of the repository in the LICENSE file.

use std::ops::Deref;
use windows::core::{PCWSTR, PWSTR};
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, FILE_ACCESS_RIGHTS, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
};

pub(crate) unsafe fn to_utf8_str(p: PWSTR) -> String {
    if p.is_null() {
        return String::new();
    }

    p.to_string().unwrap_or_else(|_e| {
        sysinfo_debug!("Failed to convert to UTF-16 string: {}", _e);
        String::new()
    })
}

pub(crate) struct HandleWrapper(pub(crate) HANDLE);

impl HandleWrapper {
    #[cfg(feature = "system")]
    pub(crate) fn new(handle: HANDLE) -> Option<Self> {
        if handle.is_invalid() {
            None
        } else {
            Some(Self(handle))
        }
    }

    pub(crate) unsafe fn new_from_file(
        drive_name: &[u16],
        open_rights: FILE_ACCESS_RIGHTS,
    ) -> Option<Self> {
        let lpfilename = PCWSTR::from_raw(drive_name.as_ptr());
        let handle = CreateFileW(
            lpfilename,
            open_rights.0,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            None,
            OPEN_EXISTING,
            Default::default(),
            HANDLE::default(),
        )
        .ok()?;
        Some(Self(handle))
    }
}

impl Deref for HandleWrapper {
    type Target = HANDLE;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for HandleWrapper {
    fn drop(&mut self) {
        let _err = unsafe { CloseHandle(self.0) };
    }
}
