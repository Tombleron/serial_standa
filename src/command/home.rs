use serde::{Deserialize, Serialize};

use super::{StandaCommand, StandaGetSetCommand};

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug)]
pub struct HomeParameters {
    pub fast_home: u32,
    pub u_fast_home: u8,
    pub slow_home: u32,
    pub u_slow_home: u8,
    pub home_delta: i32,
    pub u_home_delta: i16,
    pub home_flags: u16,
}

impl<'a> StandaCommand<'a> for HomeParameters {
    const RESERVED_BYTES: &'a [u8] = &[0; 9];
}

impl<'a> StandaGetSetCommand<'a> for HomeParameters {
    const GET_CMD_NAME: &'static str = "ghom";
    const SET_CMD_NAME: &'static str = "shom";
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug)]
pub struct HOME {}

impl<'a> StandaCommand<'a> for HOME {
    const HAS_CRC: bool = false;
    const RESERVED_BYTES: &'a [u8] = &[0; 0];
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ZERO {}

impl<'a> StandaCommand<'a> for ZERO {
    const HAS_CRC: bool = false;
    const RESERVED_BYTES: &'a [u8] = &[0; 0];
}
