use serde::{Deserialize, Serialize};

use super::{StandaCommand, StandaGetSetCommand};

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug)]
pub struct MOVEParameters {
    speed: u32,
    u_speed: u8,
    accel: u16,
    decel: u16,
    antiplay_speed: u32,
    u_antiplay_speed: u8,
    move_flags: u8,
}

impl<'a> StandaCommand<'a> for MOVEParameters {
    const RESERVED_BYTES: &'static [u8] = &[0; 9];
}

impl<'a> StandaGetSetCommand<'a> for MOVEParameters {
    const GET_CMD_NAME: &'static str = "gmov";
    const SET_CMD_NAME: &'static str = "smov";
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug)]
pub struct MOVE {
    pub position: i32,
    pub u_position: i16,
}

impl<'a> StandaCommand<'a> for MOVE {
    const HAS_CRC: bool = true;
    const RESERVED_BYTES: &'a [u8] = &[0; 6];
}
