pub mod home;

use std::{
    io::{self, Error, ErrorKind, Read, Write},
    mem::size_of,
};

use bincode::deserialize;
use serde::{Deserialize, Serialize};
use serialport::TTYPort;

fn crc16(pbuf: &[u8]) -> u16 {
    let mut crc: u16 = 0xffff;
    for &byte in pbuf {
        crc ^= byte as u16;
        for _ in 0..8 {
            let a = crc;
            let carry_flag = a & 0x0001;
            crc >>= 1;
            if carry_flag == 1 {
                crc ^= 0xa001;
            }
        }
    }
    crc
}

#[repr(C, packed)]
#[derive(Deserialize, Debug)]
struct Response<T> {
    cmd: u32,
    #[serde(flatten)]
    data: T,
    crc: u16,
}

#[repr(C, packed)]
#[derive(Serialize, Debug)]
struct Request {
    cmd: u32,
}

pub trait StandaCommand<'a>: Serialize + Sized {
    const HAS_CRC: bool = true;
    const RESERVED_BYTES: Option<&'a [u8]> = None;

    const SIZE: usize = size_of::<Self>() + size_of::<[u8; 4]>() + size_of::<u32>();

    fn as_bytes(&self, cmd_name: &'static str) -> Vec<u8> {
        let command = cmd_name.as_bytes();

        let bytes = bincode::serialize(self).expect("failed to serialize struct.");

        let mut buffer = Vec::with_capacity(Self::SIZE);

        buffer.extend_from_slice(command);
        buffer.extend_from_slice(&bytes);

        if let Some(reserved) = Self::RESERVED_BYTES {
            buffer.extend_from_slice(reserved);
        }

        if Self::HAS_CRC {
            let crc = crc16(&bytes);
            buffer.extend_from_slice(&[crc as u8, (crc >> 8) as u8]);
        }

        buffer
    }
}

pub trait StandaGetSetCommand<'a>
where
    Self: Sized,
{
    const GET_CMD_NAME: &'static str;
    const SET_CMD_NAME: &'static str;

    const SIZE: usize = size_of::<Self>() + size_of::<[u8; 4]>() + size_of::<u32>();

    fn get(port: &mut TTYPort) -> io::Result<Self>
    where
        Self: for<'de> Deserialize<'de>,
    {
        let name = Self::GET_CMD_NAME.as_bytes();

        port.write_all(name)?;

        // FIXME: move to slice
        let mut serial_buf = vec![0; Self::SIZE];
        port.read_exact(serial_buf.as_mut_slice())?;

        let response = deserialize::<Response<Self>>(&serial_buf).map_err(|_| {
            Error::new(
                ErrorKind::InvalidData,
                "failed to parse response from serial port.",
            )
        })?;

        Ok(response.data)
    }

    fn set(&self, port: &mut TTYPort) -> io::Result<()>
    where
        Self: StandaCommand<'a>,
    {
        let bytes = self.as_bytes(Self::SET_CMD_NAME);

        port.write_all(&bytes)?;

        let mut serial_buf: [u8; 4] = [0; 4];
        port.read_exact(&mut serial_buf)?;

        match serial_buf == Self::SET_CMD_NAME.as_bytes() {
            true => Ok(()),
            false => {
                let err = std::str::from_utf8(&serial_buf).unwrap_or("unknown error");
                Err(Error::new(ErrorKind::Other, err))
            }
        }
    }
}
