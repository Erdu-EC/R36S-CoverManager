use crate::custom_windows::enums::DriveTypes;
use crate::custom_windows::utils;
use crate::custom_windows::utils::pcwstr;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::{Path, PathBuf};
use windows::core::Error;
use windows::Win32::Foundation::MAX_PATH;
use windows::Win32::Storage::FileSystem;
use windows::Win32::Storage::FileSystem::GetVolumeInformationW;

pub fn get_logical_devices() -> Vec<PathBuf> {
    let letters = &mut [0; 130];
    unsafe {
        let result_length = FileSystem::GetLogicalDriveStringsW(Option::from(letters.as_mut()));
        if result_length > letters.len() as u32 {
            panic!("Error getting logical devices: {}", result_length);
        }
    };

    letters
        .split(|c| *c == 0x00)
        .filter(|s| !s.is_empty())
        .map(|c| PathBuf::from(OsString::from_wide(c)))
        .collect::<Vec<_>>()
}

pub fn get_removable_devices() -> Vec<PathBuf> {
    let drives = get_logical_devices();
    drives.into_iter()
        .filter(|d| unsafe { FileSystem::GetDriveTypeW(pcwstr::from_path_buf(d.to_path_buf())) } == DriveTypes::DriveRemovable as u32)
        .collect::<Vec<_>>()
}

pub struct LogicalDevice {
    pub(crate) path: PathBuf,
}

impl From<&Path> for LogicalDevice {
    fn from(path: &Path) -> Self {
        Self { path: path.to_path_buf() }
    }
}

impl LogicalDevice {
    /*
    pub fn get_volume_name(&self) -> OsString {
        let identifier = &mut [0u16; 100];

        unsafe {
            FileSystem::GetVolumeNameForVolumeMountPointW(
                pcwstr::from_path(self.path.as_path()),
                identifier.as_mut(),
            )
                .expect("Error getting logical device identifier");
        }

        OsString::from_wide(utils::trim_wide_null(identifier))
    }
    */

    /*
    pub fn get_device_name(&self) -> WideString {
        let os_path = self.path.as_os_str();
        let trailing_path = os_path.encode_wide().take(os_path.len() - 1).collect::<Vec<_>>();

        let mut label = WideString::new(MAX_PATH as usize);
        unsafe {
            let count = QueryDosDeviceW(pcwstr::from_vec_u16(trailing_path), Some(label.as_mut_u16()));
        }

        label
    }
    */

    pub fn get_label(&self) -> Result<OsString, Error> {
        let volume_name = &mut [0u16; MAX_PATH as usize];
        let file_system_name = &mut [0u16; MAX_PATH as usize];

        let mut volume_serial_number = 0u32;
        let mut max_component_length = 0u32;
        let mut file_system_flags = 0u32;

        let result = unsafe {
            GetVolumeInformationW(
                pcwstr::from_path(self.path.as_path()),
                Some(volume_name.as_mut()),
                Some(&mut volume_serial_number),
                Some(&mut max_component_length),
                Some(&mut file_system_flags),
                Some(file_system_name.as_mut()),
            )
        };

        match result {
            Ok(_) => Ok(OsString::from_wide(utils::trim_wide_null(volume_name))),
            Err(e) => Err(e),
        }
    }
}
