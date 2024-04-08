pub mod home;

use serde::{Deserialize, Serialize};

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

pub trait StandaGetSetCommand<'a>: StandaCommand<'a> {

    

    fn get() -> Self {
        
    }
}
