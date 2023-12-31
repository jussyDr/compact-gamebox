#![warn(missing_docs)]

//! A compacted format for storing GameBox files.

use std::io::{self, Read, Result, Write};

/// Convert a GameBox file from the given `reader` to a compacted GameBox file at the given `writer`.
pub fn compact(mut reader: impl Read, mut writer: impl Write) -> Result<()> {
    io::copy(&mut reader, &mut writer)?;

    Ok(())
}

/// Convert a compacted GameBox file from the given `reader` to a GameBox file at the given `writer`.
pub fn decompact(mut reader: impl Read, mut writer: impl Write) -> Result<()> {
    io::copy(&mut reader, &mut writer)?;

    Ok(())
}
