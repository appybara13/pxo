use std::{io, str};

use crate::error::PxoError;

pub(crate) trait ReadExt: io::Read {
    fn read_string(&mut self, length: usize) -> Result<String, PxoError> {
        let mut bytes = vec![0u8; length];
        self.read_exact(&mut bytes)?;
        let str = str::from_utf8(&bytes)?;
        Ok(str.to_string())
    }

    fn read_u32(&mut self) -> Result<u32, io::Error> {
        let mut bytes = [0u8; 4];
        self.read_exact(&mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }
}
