use std::{collections::HashSet, error::Error};

use bitvec::prelude::*;
use windows::Win32::{Foundation::GetLastError, Storage::FileSystem::GetLogicalDrives};

// Only the 26 chars of the alphabet are allowed
const INVALID_DRIVE_LETTER_BITMASK: u32 = 0b11111100_00000000_00000000_00000000;

pub(crate) fn get_available_drive_names() -> Result<HashSet<char>, LogicalDrivesError> {
    let bit_mask = unsafe { GetLogicalDrives() };

    if bit_mask == 0 {
        let error = unsafe { GetLastError() };
        Err(LogicalDrivesError::Api(error.0))
    } else if bit_mask & INVALID_DRIVE_LETTER_BITMASK != 0 {
        Err(LogicalDrivesError::InvalidNumberOfDrives)
    } else {
        Ok(bit_mask
            .view_bits::<Lsb0>()
            .iter()
            .zip('A'..='Z')
            .filter_map(|(bit, name)| if *bit { Some(name) } else { None })
            .collect())
    }
}

pub enum LogicalDrivesError {
    InvalidNumberOfDrives,
    Api(u32),
}

impl Error for LogicalDrivesError {}
impl std::fmt::Debug for LogicalDrivesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicalDrivesError::Api(code) => write!(f, "Api Error: {code}"),
            LogicalDrivesError::InvalidNumberOfDrives => write!(f, "Invalid Number of Drives."),
        }
    }
}
impl std::fmt::Display for LogicalDrivesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
