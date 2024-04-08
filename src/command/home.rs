use serde::{Deserialize, Serialize};

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug)]
struct HomeParameters {
    fast_home: u32,
    u_fast_home: u8,
    slow_home: u32,
    u_slow_home: u8,
    home_delta: i32,
    u_home_delta: i16,
    home_flags: u16,
}
