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

pub trait StandaCommand<'a>: Serialize {
    const HAS_CRC: bool = true;
    const RESERVED_BYTES: Option<&'a [u8]> = None;
    const CMD_NAME: &'static str;

    fn calc_crc(&self) -> u16 {
        let bytes = bincode::serialize(self).unwrap();
        crc16(&bytes)
    }

    fn as_bytes(&self) -> Vec<u8> {
        let command = Self::CMD_NAME.as_bytes();
        let params = bincode::serialize(self).unwrap();

        let mut bytes = Vec::new();

        bytes.extend_from_slice(command);
        bytes.extend_from_slice(&params);

        if let Some(reserved) = Self::RESERVED_BYTES {
            bytes.extend_from_slice(reserved);
        }

        if Self::HAS_CRC {
            let crc = self.calc_crc();
            bytes.extend_from_slice(&[crc as u8, (crc >> 8) as u8]);
        }

        bytes
    }
}

pub trait StandaGetSetCommand<'a>
where
    Self: Sized,
{
    const GET_CMD_NAME: &'static str;
    const SET_CMD_NAME: &'static str;

    const SIZE: usize = size_of::<Self>() + size_of::<[u8; 4]>() + size_of::<u32>();

    fn get(mut port: TTYPort) -> io::Result<Self>
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
}
