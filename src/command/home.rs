use serde::{Deserialize, Serialize};

use super::StandaGetSetCommand;

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug)]
pub struct HomeParameters {
    fast_home: u32,
    u_fast_home: u8,
    slow_home: u32,
    u_slow_home: u8,
    home_delta: i32,
    u_home_delta: i16,
    home_flags: u16,
}

impl<'a> StandaGetSetCommand<'a> for HomeParameters {
    const GET_CMD_NAME: &'static str = "ghom";

    const SET_CMD_NAME: &'static str = "shom";
}
