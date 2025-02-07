use std::collections::HashSet;

use bitvec::prelude::*;
use windows::Win32::{Foundation::GetLastError, Storage::FileSystem::GetLogicalDrives};

use crate::error::FinderError;

// Only the 26 chars of the alphabet are allowed
const INVALID_DRIVE_LETTER_BITMASK: u32 = 0b11111100_00000000_00000000_00000000;

pub fn get_available_drive_names() -> Result<HashSet<char>, FinderError> {
    let bit_mask = unsafe { GetLogicalDrives() };

    if bit_mask == 0 {
        let error = unsafe { GetLastError() };
        Err(FinderError::DrivesApi(error.0))
    } else if bit_mask & INVALID_DRIVE_LETTER_BITMASK != 0 {
        Err(FinderError::DrivesInvalidNumberOfDrives)
    } else {
        Ok(bit_mask
            .view_bits::<Lsb0>()
            .iter()
            .zip('A'..='Z')
            .filter_map(|(bit, name)| if *bit { Some(name) } else { None })
            .collect())
    }
}

