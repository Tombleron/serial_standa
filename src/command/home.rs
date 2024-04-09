use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use super::{StandaCommand, StandaGetSetCommand};

  	// 0x1 - HOME_DIR_FIRST 	
  	// 0x2 - HOME_DIR_SECOND 
  	// 0x4 - HOME_MV_SEC_EN 
  	// 0x8 - HOME_HALF_MV 
  	// 0x30 - HOME_STOP_FIRST_BITS 	
  	// 0x10 - HOME_STOP_FIRST_REV 
  	// 0x20 - HOME_STOP_FIRST_SYN
  	// 0x30 - HOME_STOP_FIRST_LIM 	
  	// 0xc0 - HOME_STOP_SECOND_BITS 
  	// 0x40 - HOME_STOP_SECOND_REV 
  	// 0x80 - HOME_STOP_SECOND_SYN
  	// 0xc0 - HOME_STOP_SECOND_LIM
  	// 0x100 - HOME_USE_FAST 

bitflags! {
    #[derive(Serialize, Deserialize, Debug, Clone, Copy)]
    #[serde(transparent)]
    pub struct HomeFlags: u16 {
        const DIR_FIRST = 0x1;
        const DIR_SECOND = 0x2;
        const MV_SEC_EN = 0x4;
        const HALF_MV = 0x8;
        const STOP_FIRST_REV = 0x10;
        const STOP_FIRST_SYN = 0x20;
        const STOP_FIRST_LIM = 0x30;
        const STOP_SECOND_REV = 0x40;
        const STOP_SECOND_SYN = 0x80;
        const STOP_SECOND_LIM = 0xc0;
        const USE_FAST = 0x100;
    }
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug)]
pub struct HomeParameters {
    pub fast_home: u32,
    pub u_fast_home: u8,
    pub slow_home: u32,
    pub u_slow_home: u8,
    pub home_delta: i32,
    pub u_home_delta: i16,
    pub home_flags: HomeFlags,
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
